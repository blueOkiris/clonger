# Clonger

## Description

A tool for designing conlangs

## Features

The general idea is drawn out in the file [Conlang Creation Tool Design.pdf](./docs/Conlang Creation Tool Design.pdf)

Essentially, there's
 - an area that allows you to type IPA symbols easily and then copy them into various areas
 - an area that allows you to document your language's grammar and stuff in a formatted way using italics, bold, underline, tables, and images
 - an area that allows you to provide examples of your language
 - an area that acts as a dictionary
 
## Running from Souce

In the project's root folder, run the command `dotnet run`

## Current Planned Changes

 - More complex dictionaries using a drop down to see (optional) aditional information

## User's Guide

#### General Application Control

You'll notice three sections when starting up the application:

the file management area,

![file area](./docs/users-guide-img/general-app-save-bar.png)

the view switching area,

![switch area](./docs/users-guide-img/general-app-switch-bar.png)

and the view area.

![view area](./docs/users-guide-img/general-app-view.png)

These may be fairly self-explanatory, but I'll go over them regardless.

__*File Management:*__
The file management area contains four buttons:
 - New which clears out all data in the application and all text fields. It also resets the internal "current file name" which can be seen in the Title bar
 - Save which saves changes to the current file and will ask you for a new current file name if one has not been set yet.
 - Save As which is like save, but always asks for a new current file name
 - Open which will populate the application with data from a selected file

__*View Switching Area*__
There are four main views in the application, each doing various tasks, and we'll get to their function later, but the view switching area contains four buttons which, when clicked, will change the view area to reflect the current view.

__*View Area*__
This is the meat and potatoes of the application. This is where all the work is done. Each different view for the application appears here with all its controls.

Moving on, I'll now explain the various application views in order by the view switching area's buttons from right-to-left

#### IPA Typing Tool

If you click the "IPA" button in the view switching area or simply start the application, you'll be met with the IPA Typing Tool View as seen in the View Area's example image.

Here, you'll notice two areas:

a hotkey area
![hotkey area](./docs/users-guide-img/ipa-hotkey.png)

and an input area
![input area](./docs/users-guide-img/ipa-input.png)

If you type normally in the input area, you may not notice the point of this view. In order to understand, you must press and hold the alt key and one of the keys on the left side of one of the items in the hotkey view.

Let's use 'a' as an example. Here's what it lists in the hotkey view:
![a hotkey](./docs/users-guide-img/ipa-a-hotkey.png)

If we type an 'a' normally in the input area, we'll simply get an 'a'
![just an a](./docs/users-guide-img/ipa-just-a.png)

However, if we now press Alt+a, we'll type a different character:
![a alt](./docs/users-guide-img/ipa-alt-1.png)

Now, if we let go and press Alt+a again, we'll get that same character,
![a alt 2](./docs/users-guide-img/ipa-alt-2-1.png)

but if keep holding Alt and press a again, then we'll begin to cycle through the special characters listed after a in the hotkey table!
![a alt 22](./docs/users-guide-img/ipa-alt-2-2.png)
![a alt 23](./docs/users-guide-img/ipa-alt-2-3.png)

So we can type all of those characters just by pressing Alt+a over and over again, like an old T9 phone keyboard
![a alt all](./docs/users-guide-img/ipa-a-all.png)

Note that if you hold Alt and switch hotkey, it will place a new character starting from the beginning with the new hotkey. It won't replace the current hotkey as it does if you repeatedly press your chosen character. This makes it so you can type faster.

So if I'm on the third Alt+a and then switch to Alt+b, it will leave that character and place down the first Alt+b as if I had let go and pressed Alt+b. If I switch back to Alt+a, then it'll leave the Alt+b character alone, and reset to the first of a.

That's pretty much it. From here on, you type your characters, then copy and paste them where you need them.
