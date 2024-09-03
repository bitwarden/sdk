package com.bitwarden.sdk;

public class BitwardenClientException extends RuntimeException {

    public BitwardenClientException(String message) {
        super(message);
    }

    public BitwardenClientException(String message, Exception ex) {
        super(message, ex);
    }
}
