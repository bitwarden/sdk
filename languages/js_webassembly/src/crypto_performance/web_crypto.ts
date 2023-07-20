const iterations = 5000;
const keySize = 256;

const encIv = 'FX7Y3pYmcLIQt6WrKc62jA==';
const encCt = 'EDlxtzpEOfGIAIa8PkCQmA==';

export async function makeDerivedKey() {
  const importedKey = await window.crypto.subtle.importKey(
    'raw', fromUtf8('mypassword'), {
    name: 'PBKDF2'
  },
    false, ['deriveKey', 'deriveBits']
  );
  return await window.crypto.subtle.deriveKey({
      'name': 'PBKDF2',
      salt: fromUtf8('a salt'),
      iterations: iterations,
      hash: {
        name: 'SHA-256'
      }
    },
      importedKey, {
      name: 'AES-CBC',
      length: keySize
    },
      true, ['encrypt', 'decrypt']
  )
}

export async function makeKey() {
  const derivedKey = await makeDerivedKey();
  return await window.crypto.subtle.exportKey('raw', derivedKey)
}

export async function encrypt(message: string, webcryptoKey: CryptoKey) {
  var ivBytes = window.crypto.getRandomValues(new Uint8Array(16));
  const encrypted = await window.crypto.subtle.encrypt({
    name: 'AES-CBC',
    iv: ivBytes
  }, webcryptoKey, fromUtf8(message))

  var ivResult = toB64(ivBytes);
  var ctResult = toB64(encrypted);
}

export async function decrypt(webCryptoKey: CryptoKey) {
  var ivBytes = fromB64(encIv);
  var ctBytes = fromB64(encCt);

  const decrypted = await window.crypto.subtle.decrypt({
    name: 'AES-CBC',
    iv: ivBytes
  }, webCryptoKey, ctBytes)
  var result = toUtf8(decrypted);
}

function fromUtf8(str: string) {
  var strUtf8 = unescape(encodeURIComponent(str));
  var ab = new Uint8Array(strUtf8.length);
  for (var i = 0; i < strUtf8.length; i++) {
    ab[i] = strUtf8.charCodeAt(i);
  }
  return ab;
}

function toUtf8(buf: ArrayBuffer) {
  var bytes = new Uint8Array(buf);
  var encodedString = String.fromCharCode.apply(null, bytes),
    decodedString = decodeURIComponent(escape(encodedString));
  return decodedString;
}

function fromB64(str: string) {
  var binary_string = window.atob(str);
  var len = binary_string.length;
  var bytes = new Uint8Array(len);
  for (var i = 0; i < len; i++) {
    bytes[i] = binary_string.charCodeAt(i);
  }
  return bytes.buffer;
}

function toB64(buf: ArrayBuffer) {
  var binary = '';
  var bytes = new Uint8Array(buf);
  var len = bytes.byteLength;
  for (var i = 0; i < len; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return window.btoa(binary);
}
