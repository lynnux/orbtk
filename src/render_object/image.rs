use orbgl_api::Canvas;

use crate::{
    backend::Renderer,
    properties::{Bounds, Image},
    render_object::RenderObject,
    structs::{Point, Size},
    widgets::Context,
};

/// Used to render an image.
pub struct ImageRenderObject;

impl Into<Box<dyn RenderObject>> for ImageRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for ImageRenderObject {
    fn render(
        &self,
        canvas: &mut Canvas,
        _renderer: &mut dyn Renderer,
        context: &mut Context<'_>,
        global_position: &Point,
    ) {
        let mut widget = context.widget();
        let bounds = widget.clone::<Bounds>();

        if let Some(image) = widget.try_get_mut::<Image>() {
            canvas.draw_image_with_size(&mut (image.0).0, global_position.x, global_position.y, bounds.width(), bounds.height());
        }
    }
}
