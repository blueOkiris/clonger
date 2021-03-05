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

