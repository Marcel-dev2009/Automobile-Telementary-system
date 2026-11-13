use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};
pub fn _burgundy_theme() -> RenderConfig<'static> {
   let burgundy = Color::Rgb { r: 128, g: 0, b: 32 };
   let soft_burgundy = Color::Rgb { r: 165, g: 42, b: 68 };
   let mut config = RenderConfig::default_colored();
   config.prompt_prefix = Styled::new("?").with_fg(burgundy);
   config.highlighted_option_prefix = Styled::new(">").with_fg(burgundy);
   config.selected_option = Some(
   StyleSheet::new().with_fg(burgundy).with_attr(Attributes::BOLD),
   );
   config.answer = StyleSheet::new()
        .with_fg(soft_burgundy)
        .with_attr(Attributes::BOLD);
  config
}