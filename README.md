# Clonger

## Description

The ultimate tool for designing conlangs, all in one.

Type IPA easily, make notes on your conlang's history and special features, and reference created words in an extensive dictionary system.

## Build

Using [Cargo](https://www.rust-lang.org/tools/install), run `cargo build --release`

## Architecture

- Plugin based
  - Create a primary window and build other functionality through plugins
  - Keyboard popup, Documentation view, and Dictionary view all plugins
- IPA Keyboard
  - Typing IPA symbols on a computer can be annoying
  - Using a tool like [this one](https://ipa.typeit.org/full/) makes it easier
  - I plan to produce something similar that will be in a floating window within the main application window
    * Buttons to click on symbols
    * Alt+Shift will allow typing 
    * Types into a box which can then be copied
- Documentation
  - Documentation is best via LaTeX or Markdown
  - Support limited markdown functionality with support for plugins that can replace the default document section
  - Used for noting sound changes, lore, phonology, etc
- Dictionary
  - Not just translation
  - Supports current pronunciation, meaning, and part of speech
  - Feature: additional notes like declension/conjugation as well as irregularities
  - Feature: etymological history of word including previous pronuniation, meaning, and part of speech in an expanding table
- Examples
  - Table of entries which take conlang sentence, ipa transcription, gloss, and base language transcription

TODO:
- [ ] - Create main window
  - [x] - Basic window
  - [ ] - Create menu bar
  - [ ] - Create file management (new, open, save, save as, and change tracking)
  - [ ] - Create tab page system
- [ ] - Create plugin system
  - [ ] - Choose events and define plugin system for pages (Dict, Ex, Doc)
  - [ ] - Choose events and define plugin system for popup windows
- [ ] - Create keyboard plugin
  - [ ] - Create popup
  - [ ] - Add buttons and implement typing
  - [ ] - Add keyboard shortcuts
- [ ] - Create documentation plugin
  - [ ] - Create tab page
  - [ ] - Implement text field and undo/redo
  - [ ] - Implement preview within text field, i.e. \*\<word\>\* shows up as *\*\<word\>\** and \_\_\<word\>\_\_ shows up as __\_\_\<word\>\_\___
  - [ ] - Implement integration with saving and opening
- [ ] - Create dictionary plugin
  - [ ] - create tab page
  - [ ] - Implement basic entry
  - [ ] - Implement additional info
  - [ ] - Implement etymology view
  - [ ] - integrate with saving and opening
- [ ] - Create example plugin
  - [ ] - create tab page
  - [ ] - Implement example entry
  - [ ] - Implement bolding and italics of conlang and base language, respectively
  - [ ] - integrate with saving and opening
- [ ] - Package everything together neatly
  - [ ] - Figure out how to build plugins along with main app
  - [ ] - Figure out how to make a package for various systems (namely Arch, Flatpak, and Windows)

## Sub-licenses

__Symbola Font__

This project makes use of the old, free version of the [Symbola Font](https://fontlibrary.org/en/font/symbola), which is in the Public Domain CC0-1.0.

There is a [newer version](https://dn-works.com/ufas/) licensed only for personal use, but we do not make use of this version
