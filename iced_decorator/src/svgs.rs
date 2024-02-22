use iced::widget::svg;

pub fn close_svg() -> svg::Handle {
    svg::Handle::from_memory(
        "<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 16 16'><path fill='currentColor' fill-rule='evenodd' d='m8 8.707l3.646 3.647l.708-.707L8.707 8l3.647-3.646l-.707-.708L8 7.293L4.354 3.646l-.707.708L7.293 8l-3.646 3.646l.707.708L8 8.707z' clip-rule='evenodd'/></svg>
  "
        .as_bytes(),
    )
}
pub fn minimize_svg() -> svg::Handle {
    svg::Handle::from_memory(
        "
        <svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 16 16'><path fill='currentColor' d='M14 8v1H3V8h11z'/></svg>
  "
        .as_bytes(),
    )
}
pub fn maximize() -> svg::Handle {
    svg::Handle::from_memory(
        "
        <svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 16 16'><path fill='currentColor' d='M3 3v10h10V3H3zm9 9H4V4h8v8z'/></svg>
      ".as_bytes()
,
    )
}
pub fn restore() -> svg::Handle {
    svg::Handle::from_memory(
        "
        <svg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 16 16'><g fill='currentColor'><path d='M3 5v9h9V5H3zm8 8H4V6h7v7z'/><path fill-rule='evenodd' d='M5 5h1V4h7v7h-1v1h2V3H5v2z' clip-rule='evenodd'/></g></svg>
      ".as_bytes()
,
    )
}
