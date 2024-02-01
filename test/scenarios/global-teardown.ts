import { type FullConfig } from '@playwright/test';
import dotenv from 'dotenv';
import dotenvExpand from 'dotenv-expand';
import { exec } from 'node:child_process';

const utils = require('./global-utils');

utils.loadEnv();
var kcPath = process.env.KC_SETUP_PATH;

async function globalTeardown(config: FullConfig) {
    console.log("Keycloak stopping");
    exec(`ENV=test KC_SETUP_PATH=${kcPath} docker-compose -f ${kcPath}/docker-compose.yml  --project-directory . down`);
}

export default globalTeardown;
