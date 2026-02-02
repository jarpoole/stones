// https://docs.github.com/en/actions/how-tos/secure-your-work/security-harden-deployments/oidc-in-aws

resource "aws_iam_openid_connect_provider" "github" {
  url             = "https://token.actions.githubusercontent.com"
  client_id_list  = ["sts.amazonaws.com"]
  thumbprint_list = ["6938fd4d98bab03faadb97b34396831e3780aea1"] # GitHub thumbprint
  tags            = var.tags
}

resource "aws_iam_role" "github_actions_role" {
  name = "${var.project}-github-actions-deploy-role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = aws_iam_openid_connect_provider.github.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "token.actions.githubusercontent.com:aud" : "sts.amazonaws.com",
            "token.actions.githubusercontent.com:sub" = "repo:${var.repo}:ref:refs/heads/main"
          }
        }
      }
    ]
  })
  tags = var.tags
}

resource "aws_iam_policy" "github_actions_policy" {
  name        = "${var.project}-github-actions-iam-policy"
  description = "Allow GitHub Actions to deploy to S3 + CloudFront for the ${var.project}"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "s3:PutObject",
          "s3:GetObject",
          "s3:ListBucket"
        ]
        Resource = [
          "arn:aws:s3:::${var.s3_bucket}",
          "arn:aws:s3:::${var.s3_bucket}/*"
        ]
      },
      {
        Effect = "Allow"
        Action = [
          "cloudfront:CreateInvalidation"
        ]
        Resource = "arn:aws:cloudfront::${var.account_id}:distribution/${var.cloudfront_distribution_id}"
      }
    ]
  })
  tags = var.tags
}

resource "aws_iam_role_policy_attachment" "attach_policy" {
  role       = aws_iam_role.github_actions_role.name
  policy_arn = aws_iam_policy.github_actions_policy.arn
}
