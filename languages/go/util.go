package sdk

import (
	"encoding/json"
	"fmt"
)

func checkSuccessAndError(responseStr string, v interface{}) error {
	var wrapper struct {
		Success      bool             `json:"success"`
		ErrorMessage *string          `json:"errorMessage"`
		Data         *json.RawMessage `json:"data"`
	}

	err := json.Unmarshal([]byte(responseStr), &wrapper)
	if err != nil {
		return fmt.Errorf("failed to unmarshal wrapper response: %v", err)
	}

	if !wrapper.Success {
		if wrapper.ErrorMessage != nil {
			return fmt.Errorf("API error: %s", *wrapper.ErrorMessage)
		}
		return fmt.Errorf("API error: unknown")
	}

	err = json.Unmarshal(*wrapper.Data, &v)
	if err != nil {
		return fmt.Errorf("failed to unmarshal response: %v", err)
	}

	return nil
}
