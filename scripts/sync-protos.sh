#!/bin/bash

# Configuration
PROTO_REPO="https://github.com/the-sprawl-project/sprawl-protocol.git"
PROJECT_NAME="construct-cache"
PROTO_DEST_DIR="src/proto"
LOCK_FILE="proto.lock"

# Local development path (can be set via env var)
LOCAL_PROTO_PATH="${SPRAWL_PROTOCOLS_LOCAL_PATH:-}"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Usage function
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --remote          Sync from remote GitHub repository (default)"
    echo "  --local PATH      Sync from local sprawl-protocols repository"
    echo "  --push PATH       Push current protos back to local sprawl-protocols repository"
    echo "  --help            Show this help message"
    echo ""
    echo "Environment Variables:"
    echo "  SPRAWL_PROTOCOLS_LOCAL_PATH    Default local path for sprawl-protocols repo"
    exit 1
}

# Parse arguments
MODE="remote"
while [[ $# -gt 0 ]]; do
    case $1 in
        --remote)
            MODE="remote"
            shift
            ;;
        --local)
            MODE="local"
            LOCAL_PROTO_PATH="$2"
            shift 2
            ;;
        --push)
            MODE="push"
            LOCAL_PROTO_PATH="$2"
            shift 2
            ;;
        --help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

# Validate local path if needed
if [[ "$MODE" == "local" || "$MODE" == "push" ]]; then
    if [ -z "$LOCAL_PROTO_PATH" ]; then
        echo -e "${RED}Error: Local path required for --local or --push mode${NC}"
        echo "Set SPRAWL_PROTOCOLS_LOCAL_PATH environment variable or use --local PATH"
        exit 1
    fi
    
    if [ ! -d "$LOCAL_PROTO_PATH" ]; then
        echo -e "${RED}Error: Local path does not exist: $LOCAL_PROTO_PATH${NC}"
        exit 1
    fi
    
    if [ ! -d "$LOCAL_PROTO_PATH/.git" ]; then
        echo -e "${RED}Error: Path is not a git repository: $LOCAL_PROTO_PATH${NC}"
        exit 1
    fi
fi

# Function to sync from source
sync_from_source() {
    local SOURCE_DIR=$1
    local SOURCE_TYPE=$2
    
    echo -e "${BLUE}Syncing protocol definitions for $PROJECT_NAME from $SOURCE_TYPE...${NC}"
    
    # Check if project directory exists
    if [ ! -d "$SOURCE_DIR/$PROJECT_NAME" ]; then
        echo -e "${RED}Project directory '$PROJECT_NAME' not found in $SOURCE_TYPE${NC}"
        return 1
    fi
    
    # Create destination directory
    mkdir -p "$PROTO_DEST_DIR"
    
    # Backup existing protos (in case we need to rollback)
    if [ -d "$PROTO_DEST_DIR" ] && [ "$(ls -A $PROTO_DEST_DIR/*.proto 2>/dev/null)" ]; then
        BACKUP_DIR=$(mktemp -d)
        cp "$PROTO_DEST_DIR"/*.proto "$BACKUP_DIR/" 2>/dev/null
        echo -e "${YELLOW}Backed up existing protos to: $BACKUP_DIR${NC}"
    fi
    
    # Copy project-specific proto files
    echo "Copying $PROJECT_NAME proto files..."
    if cp "$SOURCE_DIR/$PROJECT_NAME"/proto/*.proto "$PROTO_DEST_DIR/" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Copied project-specific protos"
    else
        echo -e "${YELLOW}⚠${NC} No project-specific protos found"
    fi
    
    # Copy common proto files
    if [ -d "$SOURCE_DIR/common" ]; then
        echo "Copying common proto files..."
        if cp "$SOURCE_DIR/common"/*.proto "$PROTO_DEST_DIR/" 2>/dev/null; then
            echo -e "${GREEN}✓${NC} Copied common protos"
        fi
    fi
    
    # List what was synced
    echo ""
    echo "Synced proto files:"
    ls -1 "$PROTO_DEST_DIR"/*.proto 2>/dev/null | xargs -n1 basename
    
    echo ""
    echo -e "${GREEN}Successfully synced protocol definitions from $SOURCE_TYPE!${NC}"
}

# Function to push to local repo
push_to_local() {
    local DEST_DIR=$1
    
    echo -e "${BLUE}Pushing protocol definitions to local sprawl-protocols...${NC}"
    
    if [ ! -d "$PROTO_DEST_DIR" ] || [ ! "$(ls -A $PROTO_DEST_DIR/*.proto 2>/dev/null)" ]; then
        echo -e "${RED}Error: No proto files found in $PROTO_DEST_DIR${NC}"
        exit 1
    fi
    
    # Create project directory if it doesn't exist
    mkdir -p "$DEST_DIR/$PROJECT_NAME"
    
    # Copy proto files
    echo "Copying proto files to $DEST_DIR/$PROJECT_NAME/..."
    cp "$PROTO_DEST_DIR"/*.proto "$DEST_DIR/$PROJECT_NAME/" 2>/dev/null
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} Copied proto files"
        
        # Show git status
        cd "$DEST_DIR"
        echo ""
        echo "Git status in sprawl-protocols:"
        git status --short "$PROJECT_NAME/"
        
        echo ""
        echo -e "${YELLOW}Next steps:${NC}"
        echo "  cd $DEST_DIR"
        echo "  git add $PROJECT_NAME/"
        echo "  git commit -m 'Update $PROJECT_NAME protos'"
        echo "  git push"
    else
        echo -e "${RED}Failed to copy proto files${NC}"
        exit 1
    fi
}

# Main logic
case $MODE in
    remote)
        echo -e "${BLUE}Mode: Remote sync${NC}"
        
        # Read version from lock file if it exists
        if [ -f "$LOCK_FILE" ]; then
            VERSION=$(grep -o '"version": "[^"]*"' "$LOCK_FILE" | cut -d'"' -f4)
            if [ -n "$VERSION" ]; then
                echo "Using locked version: $VERSION"
                PROTO_BRANCH="$VERSION"
            else
                PROTO_BRANCH="main"
            fi
        else
            PROTO_BRANCH="main"
            echo -e "${YELLOW}No proto.lock found, using main branch${NC}"
        fi
        
        # Create temp directory
        TEMP_DIR=$(mktemp -d)
        trap "rm -rf $TEMP_DIR" EXIT
        
        # Clone the proto repo
        echo "Cloning sprawl-protocols repository..."
        if git clone --depth 1 --branch "$PROTO_BRANCH" "$PROTO_REPO" "$TEMP_DIR" 2>/dev/null; then
            sync_from_source "$TEMP_DIR" "remote"
        else
            echo -e "${RED}Failed to clone sprawl-protocols repository${NC}"
            exit 1
        fi
        ;;
        
    local)
        echo -e "${BLUE}Mode: Local sync${NC}"
        echo "Local path: $LOCAL_PROTO_PATH"
        sync_from_source "$LOCAL_PROTO_PATH" "local"
        ;;
        
    push)
        echo -e "${BLUE}Mode: Push to local${NC}"
        echo "Local path: $LOCAL_PROTO_PATH"
        push_to_local "$LOCAL_PROTO_PATH"
        ;;
esac