import { type FullConfig } from '@playwright/test';
import { execSync } from 'node:child_process';
import fs from 'fs';
import yaml from 'js-yaml';

const utils = require('./global-utils');

utils.loadEnv();

function readCurrentVersion(){
    try {
        const vw_version_file = fs.readFileSync('temp/web-vault/vw-version.json', {
            encoding: 'utf8',
            flag: 'r'
        });

        return JSON.parse(vw_version_file)["version"];
    } catch(err) {
        console.log(`Failed to read frontend current version: ${err}`);
    }
}

function readDockerRelease(){
    try {
        const docker_settings = fs.readFileSync('../docker/DockerSettings.yaml', {
            encoding: 'utf8',
            flag: 'r'
        });

        const settings = yaml.load(docker_settings);
        return settings["oidc_web_release"];
    } catch(err) {
        console.log(`Failed to read docker frontend current version: ${err}`);
    }
}

function retrieveFrontend(){
    const vw_version = readCurrentVersion();
    const oidc_release = readDockerRelease()

    if( !oidc_release ){
        console.log("Empty docker frontend release");
        process.exit(1);
    }

    try {
        if( !vw_version || !oidc_release.endsWith(vw_version.replace("oidc_button-", "")) ){
            fs.rmSync("./temp/web-vault", { recursive: true, force: true });
            execSync(`cd temp && wget -c ${oidc_release}/oidc_button_web_vault.tar.gz  -O - | tar xz`, { stdio: "inherit" });
            console.log(`Retrieved bw_web_builds-${oidc_release}`);
        } else {
            console.log(`Using existing bw_web_builds-${oidc_release}`);
        }
    } catch(err) {
        console.log(`Failed to retrieve frontend: ${err}`);
        process.exit(1);
    }
}

function buildServer(){
    if( !fs.existsSync('temp/vaultwarden') ){
        console.log("Rebuilding server");
        execSync(`cd .. && cargo build --features sqlite,mysql,postgresql --release`, { stdio: "inherit" });
        execSync(`cp ../target/release/vaultwarden temp/vaultwarden`, { stdio: "inherit" });
    } else {
        console.log("Using existing server");
    }
}

async function globalSetup(config: FullConfig) {
    execSync("mkdir -p temp/logs");

    buildServer();
    retrieveFrontend();
}

export default globalSetup;
