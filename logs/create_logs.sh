# developer script to create job report documentation files

TOOL_SUITE="__TOOL_SUITE_NAME__"
SUITE_COMMAND="__TOOL_SUITE_SHORT_NAME__"

# get job file from arg
JOB_FILE=$1
if [ "${JOB_FILE}" == "" ]; then
    echo "usage: create_logs.sh <job_file>"
    exit 1
fi
if [ ! -f ${JOB_FILE} ]; then
    echo "job file ${JOB_FILE} not found"
    echo "usage: create_logs.sh <job_file>"
    exit 1
fi

# parse log file name
JOB_FILE_NAME=$(basename ${JOB_FILE} .yml)
LOG_DIR=../rudi/suites/developer-forks/${TOOL_SUITE}/logs
LOG_FILE=${LOG_DIR}/${JOB_FILE_NAME}.log

# initialize log file
RUDI="../rudi/rudi -d"
SEP="~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n"
echo -e "${SEP}pipline: ${JOB_FILE_NAME}"           > ${LOG_FILE}

# print status
echo -e "${SEP}${SUITE_COMMAND} status"                         >> ${LOG_FILE}
$RUDI ${JOB_FILE} status        | perl mask_logs.pl >> ${LOG_FILE}

# print log report
echo -e "${SUITE_COMMAND} report"                               >> ${LOG_FILE}
$RUDI ${JOB_FILE} report -j all | perl mask_logs.pl >> ${LOG_FILE}

# print file output listing (will prompt for job if multiple jobs)
echo -e "${SUITE_COMMAND} ls\n${SEP}"                                   >> ${LOG_FILE}
$RUDI ${JOB_FILE} ls            | perl mask_logs.pl >> ${LOG_FILE}
