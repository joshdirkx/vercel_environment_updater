# Updating Vercel Environment Variables with GitHub Actions

This project builds and uploads a Docker image to Dockerhub that, when 
executed, updates environment variables in Vercel through their API.

## Using this GitHub Action

This GitHub Action sets one variable per invocation.

```yaml
jobs:
  update_vercel_environment_variables:
    runs-on: ubuntu-latest
    
    steps:
      - uses: docker://joshdirkx/vercel_environment_updater
        env:
          VERCEL_TOKEN: ${{ secrets.VERCEL_TOKEN }}
          VERCEL_TEAM_ID: ${{ secrets.VERCEL_TEAM_ID }}
          VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}
          KEY:
          VALUE:
          TARGET_ENVIRONMENT:
          VARIABLE_TYPE:
          COMMENT:
```