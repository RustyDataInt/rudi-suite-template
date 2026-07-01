
COMMAND=${1}
SUBCOMMAND=${2}

FAST_TMP_DIR="/tmpssd"
RUDI="/nfs/turbo/path-wilsonte-turbo/rudi/rudi-suite-template/rudi/rudi"
# # export CARGO_HOME="/scratch/wilsonte_root/wilsonte0/shared_data/cargo_home"
# export CARGO_HOME="/nfs/turbo/path-wilsonte-turbo/cargo_home"
# mkdir -p ${CARGO_HOME}

exec ${RUDI} -d ${COMMAND} --fast-tmp-dir ${FAST_TMP_DIR} ${SUBCOMMAND}
