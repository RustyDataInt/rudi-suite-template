---
published: false
---

## Job file templates

The `templates` folder carries YAML job file templates
you can adapt for running __TOOL_SUITE_NAME__ pipelines on your data.

Template names reflect the pipeline and action they
correspond to, e.g., `<pipeline>_<action>.yml`.

The __TOOL_SUITE_NAME__ CLI can run job files either 
inline in the calling shell or by submitting jobs to your server job scheduler;
the latter is recommended for most use cases. Thus, our most common usage pattern is:

```sh
rudi inspect myJob.yml          # check the formatting of your job file
rudi mkdir myJob.yml            # create any missing output directories
rudi submit --dry-run myJob.yml # test the job file to see what will happen
rudi submit myJob.yml           # submit the job to Slurm or your scheduler
rudi myJob.yml status           # show the state of all submitted jobs
rudi myJob.yml top              # monitor a running job
rudi myJob.yml report           # show a job log report
rudi myJob.yml ls               # show the contents of a job's output diretory
```
