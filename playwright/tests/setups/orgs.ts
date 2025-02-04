import { expect, type Browser,Page } from '@playwright/test';

export async function create(test, page: Page, name: string) {
    await test.step('Create Org', async () => {
        await page.locator('a').filter({ hasText: 'Password Manager' }).first().click();
        await expect(page.getByTitle('All vaults', { exact: true })).toBeVisible();
        await page.getByRole('link', { name: 'New organisation' }).click();
        await page.getByLabel('Organisation name (required)').fill(name);
        await page.getByRole('button', { name: 'Submit' }).click();
        await expect(page.getByTestId('toast-title')).toHaveText('Organisation created');
        await page.locator('#toast-container').getByRole('button').click();
    });
}

export async function members(test, page: Page, name: string) {
    await test.step(`Navigate to ${name}`, async () => {
        await page.locator('a').filter({ hasText: 'Admin Console' }).first().click();
        await page.locator('org-switcher').getByLabel(/Toggle collapse/).click();
        await page.locator('org-switcher').getByRole('link', { name: `${name}` }).first().click();
        await expect(page.getByRole('heading', { name: `${name} collections` })).toBeVisible();
        await page.locator('div').filter({ hasText: 'Members' }).nth(2).click();
        await expect(page.getByRole('heading', { name: 'Members' })).toBeVisible();
    });
}

export async function invite(test, page: Page, name: string, email: string) {
    await test.step(`Invite ${email}`, async () => {
        await expect(page.getByRole('heading', { name: 'Members' })).toBeVisible();
        await page.getByRole('button', { name: 'Invite member' }).click();
        await page.getByLabel('Email (required)').fill(email);
        await page.getByRole('tab', { name: 'Collections' }).click();
        await page.getByLabel('Permission').selectOption('edit');
        await page.getByLabel('Select collections').click();
        await page.getByLabel('Options list').getByText('Default collection').click();
        await page.getByRole('button', { name: 'Save' }).click();
        await expect(page.getByTestId('toast-message')).toHaveText('User(s) invited');
        await page.locator('#toast-container').getByRole('button').click();
    });
}

export async function confirm(test, page: Page, name: string, user_name: string) {
    await test.step(`Confirm ${user_name}`, async () => {
        await expect(page.getByRole('heading', { name: 'Members' })).toBeVisible();
        await page.getByRole('row', { name: user_name }).getByLabel('Options').click();
        await page.getByRole('menuitem', { name: 'Confirm' }).click();
        await expect(page.getByRole('heading', { name: 'Confirm user' })).toBeVisible();
        await page.getByRole('button', { name: 'Confirm' }).click();
        await expect(page.getByTestId('toast-message')).toHaveText(/confirmed/);
        await page.locator('#toast-container').getByRole('button').click();
    });
}

export async function revoke(test, page: Page, name: string, user_name: string) {
    await test.step(`Revoke ${user_name}`, async () => {
        await expect(page.getByRole('heading', { name: 'Members' })).toBeVisible();
        await page.getByRole('row', { name: user_name }).getByLabel('Options').click();
        await page.getByRole('menuitem', { name: 'Revoke access' }).click();
        await expect(page.getByRole('heading', { name: 'Revoke access' })).toBeVisible();
        await page.getByRole('button', { name: 'Revoke access' }).click();
        await expect(page.getByTestId('toast-message')).toHaveText(/Revoked organisation access/);
        await page.locator('#toast-container').getByRole('button').click();
    });
}
