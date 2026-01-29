#!/bin/bash
# Cleanup branch after workflow completion/rejection
# This hook is called when a workflow completes or is rejected

set -e

BRANCH="${SPETS_BRANCH}"

if [ -z "$BRANCH" ]; then
  echo "No branch to cleanup"
  exit 0
fi

# Skip if on protected branches
if [[ "$BRANCH" == "main" || "$BRANCH" == "master" || "$BRANCH" == "develop" ]]; then
  echo "Skipping cleanup of protected branch: $BRANCH"
  exit 0
fi

echo "Cleaning up branch: $BRANCH"

# Switch to main/master before deleting
git checkout main 2>/dev/null || git checkout master 2>/dev/null || {
  echo "Warning: Could not switch to main/master branch"
}

# Delete local branch
git branch -D "$BRANCH" 2>/dev/null || {
  echo "Warning: Could not delete local branch $BRANCH"
}

# Delete remote branch (optional - uncomment if needed)
# git push origin --delete "$BRANCH" 2>/dev/null || {
#   echo "Warning: Could not delete remote branch $BRANCH"
# }

echo "Branch cleanup completed"
