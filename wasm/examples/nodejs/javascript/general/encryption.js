const sydar = require('../../../../nodejs/sydar');

sydar.initConsolePanicHook();

(async () => {

    let encrypted = sydar.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = sydar.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
