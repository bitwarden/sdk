module example

go 1.21.1

// replace github.com/bitwarden/sdk/languages/go => github.com/cvele/sdk/languages/go v0.0.0-20231002081021-8c8bc29d0eef
replace github.com/bitwarden/sdk/languages/go => /Users/dani/sdk-go/languages/go

require (
	github.com/bitwarden/sdk/languages/go v0.0.0-00010101000000-000000000000
	github.com/gofrs/uuid v4.4.0+incompatible
)
