{-# LANGUAGE OverloadedStrings #-}
module Tui (startApp) where

import Brick.AttrMap (applyAttrMappings, AttrName, attrName)
import Brick.Main (simpleMain)
import Brick.Widgets.Border ( hBorder, vBorder, hBorderWithLabel, borderWithLabel
                            , borderAttr, vBorderAttr )
import Brick.Widgets.Border.Style   (ascii, unicode, unicodeBold, unicodeRounded
                                    , borderStyleFromChar, BorderStyle(..)
                                    , bsCornerTL, bsCornerTR, bsCornerBR, bsCornerBL
                                    , bsIntersectFull, bsIntersectL, bsIntersectR, bsIntersectT
                                    , bsIntersectB, bsHorizontal, bsVertical )
import Brick.Widgets.Center (center, vCenter)
import Brick.Widgets.Core   ( (<+>), withAttr, vLimit, hLimit, hBox, vBox, updateAttrMap
                            , withBorderStyle, txt, str, padLeftRight )
import Brick.Types (Widget)
import Brick.Util (fg, on)
import Data.Text (Text)
import Graphics.Vty (Attr, yellow, cyan, magenta, cyan, black)

styles :: [(Text, BorderStyle)]
styles =    [ ("ascii",             ascii)
            , ("unicode",           unicode)
            , ("unicode bold",      unicodeBold)
            , ("unicode rounded",   unicodeRounded)
            , ("custom",            custom)
            , ("from 'x'",          borderStyleFromChar 'x') ]

custom :: BorderStyle
custom = BorderStyle    { bsCornerTL = '/'
                        , bsCornerTR = '\\'
                        , bsCornerBR = '/'
                        , bsCornerBL = '\\'
                        , bsIntersectFull = '.'
                        , bsIntersectL = '.'
                        , bsIntersectR = '.'
                        , bsIntersectT = '.'
                        , bsIntersectB = '.'
                        , bsHorizontal = '*'
                        , bsVertical = '!' }

borderDemos :: [Widget ()]
borderDemos = mkBorderDemo <$> styles

mkBorderDemo :: (Text, BorderStyle) -> Widget ()
mkBorderDemo (styleName, sty) =
    withBorderStyle sty $
        borderWithLabel (str "Label") $
            vLimit 5 $
                vCenter $
                    padLeftRight 2 $
                        txt $ styleName <> " Style"

titleAttr :: AttrName
titleAttr = attrName "Title"

attrs :: [(AttrName, Attr)]
attrs = [ (borderAttr,      yellow `on` black)
        , (vBorderAttr,     fg cyan)
        , (vBorderAttr,     fg magenta)
        , (vBorderAttr,     fg cyan) ]

colorDemo :: Widget ()
colorDemo =
    updateAttrMap (applyAttrMappings attrs) $
        borderWithLabel (withAttr titleAttr $ str "Title") $
            hLimit 20 $ vLimit 5 $
                center $ str "Colors!"

ui :: Widget ()
ui = vBox   [ hBox borderDemos
            , hBorder
            , colorDemo
            , hBorderWithLabel (str "Horizontal Border Label")
            , (center (str "Left of Vertical Border")
                <+> vBorder
                <+> center (str "Right of Vertical Border" )) ]

startApp :: IO ()
startApp = do
    simpleMain ui

