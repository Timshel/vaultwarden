import { test, expect, type TestInfo } from '@playwright/test';
import { MailDev } from 'maildev';

import * as utils from '../global-utils';
import * as orgs from './setups/orgs';
import { logNewUser, logUser } from './setups/sso';

let users = utils.loadEnv();

let mailServer;

test.beforeAll('Setup', async ({ browser }, testInfo: TestInfo) => {
    mailServer = new MailDev({
        port: process.env.MAILDEV_SMTP_PORT,
        web: { port: process.env.MAILDEV_HTTP_PORT },
    })

    await mailServer.listen();

    await utils.startVaultwarden(browser, testInfo, {
        ORGANIZATION_INVITE_AUTO_ACCEPT: true,
        SMTP_FROM: process.env.VAULTWARDEN_SMTP_FROM,
        SMTP_HOST: process.env.MAILDEV_HOST,
        SSO_ENABLED: true,
        SSO_ONLY: true,
        SSO_ORGANIZATIONS_INVITE: true,
        SSO_ORGANIZATIONS_REVOCATION: true,
        SSO_SCOPES: 'email profile groups',
    });
});

test.afterAll('Teardown', async ({}) => {
    utils.stopVaultwarden();
    mailServer?.close();
});

test('Create user2', async ({ page }) => {
    await logNewUser(test, page, users.user2);
});

test('Create user3', async ({ page }) => {
    await logNewUser(test, page, users.user3);
});

test('Org setup', async ({ context, page }, testInfo: TestInfo) => {
    await logNewUser(test, page, users.user1);

    await orgs.create(test, page, 'Toto');
    await orgs.members(test, page, 'Toto');
    await orgs.invite(test, page, 'Toto', users.user2.email);
    await orgs.confirm(test, page, 'Toto', users.user2.name);
    await orgs.invite(test, page, 'Toto', users.user3.email);

    await orgs.create(test, page, 'All');
    await orgs.members(test, page, 'All');
    await orgs.invite(test, page, 'All', users.user2.email);
    await orgs.confirm(test, page, 'All', users.user2.name);
    await orgs.revoke(test, page, 'All', users.user2.name);
    await orgs.invite(test, page, 'All', users.user3.email);
    await orgs.revoke(test, page, 'All', users.user3.name);

    // We create test too otherwise revokation is disabled.
    await orgs.create(test, page, 'Test');
});

test('Check User2', async ({ context, page }, testInfo: TestInfo) => {
    await logUser(test, page, users.user2);
    await expect(page.getByLabel('vault: All', { exact: true })).toBeVisible();
    await expect(page.getByLabel('vault: Toto', { exact: true })).toHaveCount(0);
});

test('Check User3', async ({ context, page }, testInfo: TestInfo) => {
    await logUser(test, page, users.user3);
    await expect(page.getByLabel('vault: All', { exact: true })).toHaveCount(0);
    await expect(page.getByLabel('vault: Toto', { exact: true })).toHaveCount(0);
});

test('Check members', async ({ context, page }, testInfo: TestInfo) => {
    await logUser(test, page, users.user1);

    await test.step('Owner see all orgs', async () => {
        await expect(page.getByLabel('vault: All', { exact: true })).toBeVisible();
        await expect(page.getByLabel('vault: Test', { exact: true })).toBeVisible();
        await expect(page.getByLabel('vault: Toto', { exact: true })).toBeVisible();
    });

    await test.step('Restored access to All', async () => {
        await orgs.members(test, page, 'All');
        await expect(page.getByRole('row', { name: users.user2.name })).toBeVisible();
        await expect(page.getByRole('row', { name: users.user3.name })).toHaveText(/Needs confirmation/);
    });

    await test.step('Invited to Test', async () => {
        await orgs.members(test, page, 'Test');
        await expect(page.getByRole('row', { name: users.user2.name })).toHaveText(/Needs confirmation/);
    });

    await test.step('Revoked access to Toto', async () => {
        await orgs.members(test, page, 'Toto');
        await page.getByLabel('Member status filter').getByText('Revoked').click();
        await expect(page.getByRole('row', { name: users.user2.name })).toHaveText(/Revoked/);
        await expect(page.getByRole('row', { name: users.user3.name })).toHaveText(/Revoked/);
    });
});

test('With mapping', async ({ context, page }, testInfo: TestInfo) => {
    await utils.restartVaultwarden(page, testInfo, {
        ORGANIZATION_INVITE_AUTO_ACCEPT: true,
        SMTP_FROM: process.env.VAULTWARDEN_SMTP_FROM,
        SMTP_HOST: process.env.MAILDEV_HOST,
        SSO_ENABLED: true,
        SSO_ONLY: true,
        SSO_ORGANIZATIONS_INVITE: true,
        SSO_ORGANIZATIONS_REVOCATION: true,
        SSO_ORGANIZATIONS_ID_MAPPING: "Test:Test;All:All",
        SSO_SCOPES: 'email profile groups',
    }, true);

    await test.step('create user2', async () => {
        await logNewUser(test, page, users.user2);
    });

    await context.clearCookies();

    await test.step('org setup', async () => {
        await logNewUser(test, page, users.user1);

        await orgs.create(test, page, 'Test');
        await orgs.create(test, page, 'All');

        await orgs.create(test, page, 'Toto');
        await orgs.members(test, page, 'Toto');
        await orgs.invite(test, page, 'Toto', users.user2.email);
        await orgs.confirm(test, page, 'Toto', users.user2.name);
    });

    await context.clearCookies();

    await test.step('Org still present', async () => {
        await logUser(test, page, users.user2);
        await expect(page.getByLabel('vault: Toto', { exact: true })).toBeVisible();
    });
});
