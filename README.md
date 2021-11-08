## Pulse audio Dunst Notifications

### Dunst Notifications for Volume Changes

This simple rust script will adjust volume for the default sink, and send a notification to dunst displaying the device name, volume, and an icon displaying either muted, low volume, medium volume, or high volume.

In order to properly display icons, ensure you have the [la-capitaine icon pack](https://github.com/keeferrourke/la-capitaine-icon-theme) installed, as those are the icons the script searchs for. If you prefer another icon theme, simply modify the 'icon_path' variable on line 30 to point to the proper location of your prefer volume icons.

#### Usage

Compile the program, and then execute the binary with one of the following arguments:

- ./volume-notifications up: Increases the volume by 5%
- ./volume-notifications down: Decreases the volume by 5%
- ./volume-notifications mute: toggles between muted and unmuted
