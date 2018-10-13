# `rofi-todoist`

Add tasks to todoist, using Rofi

[![CircleCI](https://circleci.com/gh/RealOrangeOne/rofi-todoist.svg?style=svg)](https://circleci.com/gh/RealOrangeOne/rofi-todoist)

## Usage

Set `$TODOIST_API_TOKEN` to your Todoist API token.

Run the application. This will present a rofi prompt to input the task data. Entering nothing, pressing escape, or clicking off of rofi will cancel the application. The prompt supports the Todoist [Quick Add](https://get.todoist.help/hc/en-us/articles/115001745265) syntax, however doesn't handle autocomplete or formatting.

Press enter to commit the task details. Once the data has been sent to Todoist, the task name will be displayed. If there was an error, details of it will be displayed instead.
