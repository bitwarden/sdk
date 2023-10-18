package com.bitwarden.sdk;

public class BitwardenSettings {

    private String apiUrl;

    private String identityUrl;

    public BitwardenSettings() {
    }

    public BitwardenSettings(String apiUrl, String identityUrl) {
        this.apiUrl = apiUrl;
        this.identityUrl = identityUrl;
    }

    public String getApiUrl() {
        return apiUrl;
    }

    public void setApiUrl(String apiUrl) {
        this.apiUrl = apiUrl;
    }

    public String getIdentityUrl() {
        return identityUrl;
    }

    public void setIdentityUrl(String identityUrl) {
        this.identityUrl = identityUrl;
    }
}
