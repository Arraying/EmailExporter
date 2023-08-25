# EmailExporter

Are you leaving your toxic school/university/workplace? 
Do you remember what services you signed up for? No? Me neither.

EmailExporter, which doesn't actually export emails, takes a `.mbox` mailbox and breaks down all the emails you have received.
Each domain is listed, and for each domain whom you received emails from.
With this, it should be easy to determine which accounts need their email switched.

## Usage

Just run it as a single file Python script.
The script expects one argument, the path of the `.mbox` file.
If your path contains spaces that's fine, those are concatenated.
