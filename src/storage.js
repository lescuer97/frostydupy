import { invoke } from "@tauri-apps/api";


/** 
    * @desc Checks and pulls config in the case that it's present
    * @return void
    * */
export function checkAndSetupConfig() {

    invoke("check_and_setup_config").then((res) => {
        console.log({res});
    });
    // invoke("get_wallets").then((res) => {
    //     console.log({get_wallets: res});
    // });
}

/**
    * @desc Creates a wallet
    * @return string[]
    * */
export function getWallets() {

    /** @type string[] */
    let wallets = [];

     invoke("get_wallets").then((res) => {
         wallets = res;
    });
    return wallets;
}

