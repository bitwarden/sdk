import * as forge from 'node-forge';

const keySize = 256;

const encIv = 'FX7Y3pYmcLIQt6WrKc62jA==';
const encCt = 'EDlxtzpEOfGIAIa8PkCQmA==';
const forgeKey = pbkdf2(5000); // Keep this iteration count constant -- it's the key used to encrypt the message being decrypted

function getRandomForgeBytes() {
  var bytes = new Uint8Array(16);
  window.crypto.getRandomValues(bytes);
  return String.fromCharCode.apply(null, bytes);
}

export function pbkdf2(numIterations) {
  return forge.pbkdf2('mypassword', 'a salt', numIterations, keySize / 8, 'sha256');
}

export function encrypt(message: string) {
  var buffer = forge.util.createBuffer(message, 'utf8');
  var cipher = forge.cipher.createCipher('AES-CBC', forgeKey);
  var ivBytes = getRandomForgeBytes();
  cipher.start({
    iv: ivBytes
  });
  cipher.update(buffer);
  cipher.finish();
  var encryptedBytes = cipher.output.getBytes();

  var result = {
    iv: forge.util.encode64(ivBytes),
    ct: forge.util.encode64(encryptedBytes)
  };

}

export function decrypt() {
  var decIvBytes = forge.util.decode64(encIv);
  var ctBytes = forge.util.decode64(encCt);
  var ctBuffer = forge.util.createBuffer(ctBytes);

  var decipher = forge.cipher.createDecipher('AES-CBC', forgeKey);
  decipher.start({
    iv: decIvBytes
  });
  decipher.update(ctBuffer);
  decipher.finish();

  var result = decipher.output.toString('utf8');
}
