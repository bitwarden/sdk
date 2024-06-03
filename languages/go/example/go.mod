module example

replace github.com/bitwarden/sm-sdk-go => ../

go 1.21

require (
	github.com/bitwarden/sm-sdk-go v0.0.0-00010101000000-000000000000
	github.com/gofrs/uuid v4.4.0+incompatible
)
