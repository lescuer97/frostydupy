import './style.css'
import { checkAndSetupConfig, getWallets } from './src/storage.js'
import { invoke } from '@tauri-apps/api'
import walletApproval from './wallet_approval.html?raw'
import { formValuesToObject } from './src/utils_dom.js';


checkAndSetupConfig();

let wallets = getWallets();

document.querySelector('#app').innerHTML = `
  <div>
    <div id="messages">
    </div>
    <button id="create-wallet"type="button">
        create wallet
    </button>
    

  </div>
`

if (wallets.length == 0) {
    let messages = document.getElementById("messages");
    messages.innerHTML = `
        <div>
            <p>
                No wallets found, click the create button to create one
            </p>
        </div>
    `
}

/** 
    * @property {string} network
    * @property {string} name
    * */
const wallet_config = {
    network: "regtest",
    name: "test",
}

/** @type string[] */
let seed_words = [];

document.getElementById("create-wallet").addEventListener("click", () => {
    invoke("create_wallet", wallet_config).then((res) => {
        seed_words = res;

        let words = document.createElement("div");
        words.classList.add("seed-words");

        for (let i = 0; i < seed_words.length; i++) {
            let word = document.createElement("p");
            word.innerHTML = ` ${i + 1}. ${seed_words[i]}`;
            words.appendChild(word);
        }
        // const parser = new DOMParser();
        // const htmlDoc = parser.parseFromString(walletApproval, 'text/html');
        const range = document.createRange();
        const fragment = range.createContextualFragment(walletApproval);

        /** @type HTMLFormElement */
        const aprovalForm = fragment.querySelector("form");
    
        // Insert the new node at the calculated position
        aprovalForm.insertBefore(words, aprovalForm.firstChild);

        document.getElementById("app").lastChild.after(aprovalForm);
        // document.getElementById("app").lastChild.after(approvalButton);

        aprovalForm.addEventListener("submit", (e) => {
            e.preventDefault();
            let values = formValuesToObject(aprovalForm);

            invoke("approve_wallet", {words: seed_words, ...values}).then((res) => {
                console.log({res});
            });
        });
        //


    });
    
});


