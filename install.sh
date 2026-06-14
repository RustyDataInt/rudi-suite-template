#!/bin/sh

#--------------------------------------------------------------------
# a _suite's_ 'install.sh' script installs the frameworks and any external 
# suite dependencies when running in single-suite mode
#--------------------------------------------------------------------
IS_MULTI_SUITE="is-multi-suite"
IS_SINGLE_SUITE="is-single-suite"
export SUPPRESS_RUDI_BASHRC=TRUE
if [ "$1" = "--yes" ] || [ "$1" = "-y" ]; then
    RUDI_FORCE_GIT=TRUE
fi
if [ "$1" = "1" ]; then
    RUDI_FORCE_GIT=TRUE
fi

#----------------------------------------------------------------------
# root directory of this tool suite, after user cloned it from GitHub
#----------------------------------------------------------------------
export SUITE_DIR="$(cd "$(dirname "$0")" >/dev/null 2>&1 && pwd)"
export SUITE_NAME="$(basename "$SUITE_DIR")"

#----------------------------------------------------------------------
# discover whether this suite is in a is-multi-suite or is-single-suite installation
# if it is contained in a parent rudi/suites/* folder, then it is always is-multi-suite
# otherwise, RuDI will be installed and run from within this suite directory (i.e., is-single-suite)
#----------------------------------------------------------------------
export SUITE_MODE="$IS_MULTI_SUITE"
export RUDI_DIR="$SUITE_DIR/../../.."
export RUDI_TARGET="$RUDI_DIR/rudi"
if [ ! -d "$RUDI_DIR/frameworks" ] || [ ! -d "$RUDI_DIR/suites" ] || [ ! -f "$RUDI_TARGET" ]; then
    export SUITE_MODE="$IS_SINGLE_SUITE" # i.e., this is the top-level of a single-suite installation
    export RUDI_DIR="$SUITE_DIR/rudi"
    export RUDI_TARGET="$RUDI_DIR/rudi"
fi
IS_READLINK="$(which readlink 2>/dev/null)"
if [ -n "$IS_READLINK" ]; then 
    export RUDI_DIR="$(readlink -f "$RUDI_DIR")"
fi

#----------------------------------------------------------------------
# if is-multi-suite, exit quietly with a helpful message and no action taken
#----------------------------------------------------------------------
if [ "$SUITE_MODE" = "$IS_MULTI_SUITE" ] || [ -f "$RUDI_DIR/IS_SINGLE_SUITE" ]; then
    printf "\nNothing to do.\n\n"
    printf "This copy of '%s' is part of an multi-suite installation in directory:\n    %s\n\n" "$SUITE_NAME" "$RUDI_DIR"
    printf "Use it by calling the 'rudi' command line utility in that directory (multi-suite installation)\n"
    printf "or the 'run' script in the parent of that directory (single-suite installation).\n\n"
    exit 1
fi

#----------------------------------------------------------------------
# get permission to install in is-single-suite mode
#----------------------------------------------------------------------
printf "\n"
printf "----------------------------------------------------------------------\n"
printf "Welcome to the Rusty Data Interface (RuDI) installer\n"
printf "----------------------------------------------------------------------\n"
printf "\n"
printf "This script will initialize '%s' pipelines for use by cloning\n" "$SUITE_NAME"
printf "the RuDI manager utility plus any additional suite dependencies.\n"
printf "\n"
if [ -n "$RUDI_FORCE_GIT" ]; then
    PERMISSION=y
else 
    printf "Do you wish to continue? (type 'y' for 'yes'): "
    read -r PERMISSION
fi
if [ "$PERMISSION" != "y" ]; then exit; fi

#----------------------------------------------------------------------
# install/update the RuDI nested within this suite's directory
# record that is a is-single-suite installation of $SUITE_NAME
#----------------------------------------------------------------------
if [ -d "$RUDI_DIR" ]; then
    cd "$RUDI_DIR" || exit 1
    git checkout main
    git pull
else 
    git clone https://github.com/RustyDataInt/rudi.git
    cd "$RUDI_DIR" || exit 1
fi
printf "%s\n" "$SUITE_NAME" > IS_SINGLE_SUITE
"$RUDI_DIR/install.sh"

#----------------------------------------------------------------------
# programmatically write the is-single-suite installation's 'config/suites.yml' file
#----------------------------------------------------------------------
printf "writing rudi/config/suites.yml\n"
{
    printf 'suites:\n'
    grep -A 10 '^\[remote "origin"\]' "$SUITE_DIR/.git/config" | 
        grep -v '^\[remote "origin"\]' | 
        grep -B 100 '^\[' | head -n 1 |
        grep 'url' | 
        sed 's/^[[:space:]]*url[[:space:]]*=[[:space:]]*//' | 
        awk '{print "  - "$0}'
    grep -v '^[[:space:]]*#' "$SUITE_DIR/_config.yml" | 
        grep '[^[:space:]]' |
        sed -n '/^suite_dependencies:/,/^[^[:space:]]/p' | 
        grep -v '^suite_dependencies:' | 
        sed -n '/^[^[:space:]]/q;p' | 
        grep '[[:space:]]*-[[:space:]]' | 
        sed 's/^[[:space:]]*-[[:space:]]*//' | 
        awk '{print "  - "$0}'
} > "$RUDI_DIR/config/suites.yml"

#----------------------------------------------------------------------
# re-run the installation to add all required tool suites (including this one)
# installer checks out frameworks to 'latest' and this suite to:
#   latest, if called by user at command line
#   SUITE_VERSION, if called during a container build
#----------------------------------------------------------------------
"$RUDI_DIR/install.sh"

#----------------------------------------------------------------------
# clean up this suite root directory by removing most git repo files
# as they are now re-cloned within the nested installation
# leave the utilities to re-install and run the tool suite
#----------------------------------------------------------------------
printf "cleaning up this suite directory\n"
cd "$SUITE_DIR" || exit 1
rm -fr .git .github apps docs logs options pipelines templates
rm -f  .gitignore index.html overview.md rust.txt singularity.def

# finish up all successful installation paths
printf "\nInstallation complete.\n\n"
