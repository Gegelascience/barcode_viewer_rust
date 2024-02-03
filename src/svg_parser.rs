

trait ToSvgString {
    fn to_svg_string(&self) -> String;
}

pub struct SvgTag {
    pub(crate) child: GTag,
    pub(crate) version:String,
    pub(crate) base_profile:String,
    pub(crate) width:i32,
    pub(crate) height:i32,
    pub(crate) xmlns:String
}

pub struct GTag {
    pub(crate) stoke:String,
    pub(crate) children:Vec<LineTag>
}

pub struct LineTag {
    pub(crate) stroke_width:i32,
    pub(crate) x1:i32,
    pub(crate) y1:i32,
    pub(crate) x2:i32,
    pub(crate) y2:i32
}

impl ToSvgString for LineTag {
    fn to_svg_string(&self) -> String {
        let mut string_value: String = "<line".to_string();

        string_value = string_value + " stroke-width='" + &self.stroke_width.to_string() + "'" ;
        string_value = string_value + " x1='" + &self.x1.to_string() + "'" ;
        string_value = string_value + " y1='" + &self.y1.to_string() + "'" ;
        string_value = string_value + " x2='" + &self.x2.to_string() + "'" ;
        string_value = string_value + " y2='" + &self.y2.to_string() + "'" ;

        string_value = string_value + "></line>";

        return string_value
    }
}

impl ToSvgString for GTag {
    fn to_svg_string(&self) -> String {
        let mut string_value: String = "<g".to_string();

        string_value = string_value + " stroke='" + &self.stoke.to_string() + "'" ;
        string_value = string_value + " >";

        for line in &self.children  {
            string_value = string_value + &line.to_svg_string();
        }
        string_value = string_value + "</g>";

        return string_value
    }
}

impl ToSvgString for SvgTag {
    fn to_svg_string(&self) -> String {
        let mut string_value: String = "<svg".to_string();

        string_value = string_value + " version='" + &self.version.to_string() + "'" ;
        string_value = string_value + " baseProfile='" + &self.base_profile.to_string() + "'" ;
        string_value = string_value + " width='" + &self.width.to_string() + "'" ;
        string_value = string_value + " height='" + &self.height.to_string() + "'" ;
        string_value = string_value + " xmlns='" + &self.xmlns.to_string() + "'" ;
        string_value = string_value + " >";

        string_value = string_value + &self.child.to_svg_string();

        string_value = string_value + "</svg>";

        return string_value
    }
}


pub fn get_svg_string(barcode_data:String) -> String {
    let x_position = 10;
    let mut list_lines: Vec<LineTag> = Vec::new();
    let mut index = 1;
    for char in barcode_data.chars() {
        if char == '1' {
            list_lines.push(
                LineTag {
                    stroke_width:4,
                    x1:x_position + index*4,
                    y1:10,
                    x2:x_position + index*4,
                    y2:60
                }
            )
        }
        index = index + 1
    }

    let g_tag = GTag {
        stoke:"black".to_string(),
        children:list_lines
    };

    let svg_tag = SvgTag {
        child:g_tag,
        version:"1.1".to_string(),
        base_profile:"full".to_string(),
        width:700,
        height:200,
        xmlns:"http://www.w3.org/2000/svg".to_string()
    };

    return svg_tag.to_svg_string()
}