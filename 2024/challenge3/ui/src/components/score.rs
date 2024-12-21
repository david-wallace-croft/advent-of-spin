// use super::super::data::calculated::Calculated;
// use ::dioxus::dioxus_core::internal::generational_box::GenerationalRef;
// use ::dioxus::prelude::*;
// use ::std::cell::Ref;

// #[component]
// pub fn Score() -> Element {
//   let calculated_signal: Signal<Calculated> =
//     consume_context::<Signal<Calculated>>();

//   let calculated: GenerationalRef<Ref<Calculated>> =
//     calculated_signal.read_unchecked();

//   rsx! {
//     table {
//     thead {
//     tr {
//     th { "Name" }
//     th { "Score" }
//     }
//     }
//     tbody {
//     tr {
//     td { "{&calculated.name}" }
//     td { "{&calculated.score}" }
//     }
//     }
//     }
//   }
// }
