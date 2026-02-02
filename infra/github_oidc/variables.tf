variable "account_id" {
  type = string
}

variable "project" {
  type = string
}

variable "repo" {
  type = string
}

variable "tags" {
  type = map(string)
}

variable "cloudfront_distribution_id" {
  type = string
}

variable "s3_bucket" {
  type = string
}
