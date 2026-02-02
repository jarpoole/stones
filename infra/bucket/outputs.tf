output "cloudfront_distribution_id" {
  value = aws_cloudfront_distribution.cdn.id
}

output "s3_bucket" {
  value = aws_s3_bucket.site.id
}
