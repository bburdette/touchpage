
// --------------------------------------------------------
// control update messages.
// --------------------------------------------------------

/*

  code in elm.  you never get label updates from there.

  JE.object [ ("controlType", JE.string "button")
            , ("controlId", SvgThings.encodeControlId um.control_id)
            , ("updateType", encodeUpdateType um.updateType)
            ]

  JE.object [ ("controlType", JE.string "slider")
            , ("controlId", SvgThings.encodeControlId um.control_id)
            , ("updateType", encodeUpdateType um.updateType)
            , ("location", (JE.float um.location))

*/

#[derive(Debug,Clone)]
pub enum ButtonState { 
  Pressed,
  Unpressed
  }

#[derive(Debug,Clone)]
pub enum SliderState { 
  Pressed,
  Unpressed
  }

#[derive(Debug,Clone)]
pub enum UpdateMsg { 
  Button  { control_id: Vec<i32>
          , state: Option<ButtonState>
          , label: Option<String>
          },
  Slider  { control_id: Vec<i32>
          , state: Option<SliderState>
          , location: Option<f64>
          , label: Option<String>
          },
  Label   { control_id: Vec<i32>
          , label: String 
          },
}

