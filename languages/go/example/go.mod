module example

replace github.com/bitwarden/sdk/languages/go => ../

go 1.20

require (
	github.com/bitwarden/sdk/languages/go v0.0.0-00010101000000-000000000000
	github.com/gofrs/uuid v4.4.0+incompatible
)
