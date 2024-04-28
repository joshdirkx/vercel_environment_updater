# Vercel Environment Variable Updater GitHub Action

Use this GitHub Action to change environment variables inside your Vercel 
projects.

## Using this GitHub Action

This GitHub Action sets one variable per invocation.

VARIABLE_TYPE must be one of ...

TARGET_ENVIRONMENT must be one of ...

GITHUB_BRANCH may not be set when the TARGET_ENVIRONMENT is production.

To learn more about target environments, visit the Vercel documentation.

To learn more about variable types, visit the Vercel documentation.

```yaml
env:
  VERCEL_TOKEN: ${{ secrets.VERCEL_TOKEN }}
  VERCEL_TEAM_ID: ${{ secrets.VERCEL_TEAM_ID }}
  VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}

jobs:
  update_vercel_environment_variables:
    runs-on: ubuntu-latest
    
    steps:
      - uses: joshdirkx/name@latest
        env:
          NAME: ''
          VALUE: ''
          TARGET_ENVIRONMENT: ''
          GITHUB_BRANCH: ''
          VARIABLE_TYPE: ''
          COMMENT: ''
```