locals {
  tags = {
    repository = "https://github.com/jarpoole/stones"
  }
}

terraform {
  backend "s3" {
    bucket       = "jarpoole-terraform"
    key          = "stones"
    region       = "us-east-2"
    use_lockfile = true
  }
}

module "storage" {
  source = "./bucket"
  tags   = local.tags
}

module "github_oidc" {
  source                     = "./github_oidc"
  cloudfront_distribution_id = module.storage.cloudfront_distribution_id
  repo                       = "https://github.com/jarpoole/stones"
  project                    = "stones"
  s3_bucket                  = module.storage.s3_bucket
  account_id                 = "122941220524"
  tags                       = local.tags
}
