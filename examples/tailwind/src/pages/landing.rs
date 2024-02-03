use yew::prelude::*;
use yew::Callback;
use yew_alert::{Alert, AlertProps};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let show_alert_0 = use_state(|| false);
    let show_alert_1 = use_state(|| false);
    let show_alert_2 = use_state(|| false);
    let show_alert_3 = use_state(|| false);
    let show_alert_4 = use_state(|| false);
    let show_alert_5 = use_state(|| false);
    let show_alert_6 = use_state(|| false);

    let alerts = vec![AlertProps {
            message: "This is in the top-left corner.",
            timeout: 1500,
            show_alert: show_alert_0,
            title: "Top Left",
            alert_class: "w-[250px] h-56 rounded-md shadow-lg bg-gray-800 text-white border border-gray-600 p-4",
            icon_type: "warning",
            position: "top-left",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-2xl text-white font-mono",
            message_class: "text-gray-300 text-md font-serif",
            icon_class: "",
            confirm_button_text: "Ok",
            cancel_button_text: "Cancel",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "50",
        },
        AlertProps {
            message: "This is in the top of the screen.",
            timeout: 2500,
            show_alert: show_alert_1,
            title: "Top",
            alert_class: "w-96 h-64 text-center rounded-md shadow-lg bg-blue-500 text-white border border-blue-700 p-4",
            icon_type: "error",
            position: "top",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-2 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-white text-2xl",
            message_class: "text-blue-300 text-xl",
            icon_class: "flex justify-center",
            confirm_button_text: "Ok",
            cancel_button_text: "No",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: true,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "70",
        },
        AlertProps {
            message: "This is in the top-right corner.",
            timeout: 3500,
            show_alert: show_alert_2,
            title: "Top Right",
            alert_class: "w-1/4 h-64 text-center rounded-md shadow-lg bg-gray-500 text-white border border-green-700 p-4",
            icon_type: "success",
            position: "top-right",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-white text-2xl",
            message_class: "text-green-300 text-2xl",
            icon_class: "flex justify-center",
            confirm_button_text: "",
            cancel_button_text: "",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "50",
        },
        AlertProps {
            message: "This is in the middle of the screen.",
            timeout: 3500,
            show_alert: show_alert_3,
            title: "Middle",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            icon_type: "info",
            position: "middle",
            alert_class: "text-center rounded-md shadow-lg bg-yellow-500 text-white border border-yellow-700 p-4",
            title_class: "text-black",
            message_class: "text-black",
            icon_class: "flex justify-center",
            confirm_button_text: "",
            cancel_button_text: "",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "black",
            icon_width: "50",
        },
        AlertProps {
            message: "This is in the bottom-right corner.",
            timeout: 1500,
            show_alert: show_alert_4,
            title: "Bottom Right",
            alert_class: "w-96 h-56 text-center rounded-md shadow-lg bg-red-500 text-white border border-red-700 p-4",
            icon_type: "question",
            position: "bottom-right",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-white",
            message_class: "text-red-300",
            icon_class: "flex justify-center",
            confirm_button_text: "",
            cancel_button_text: "",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "50",
        },
        AlertProps {
            message: "This is in the bottom of the screen.",
            timeout: 1500,
            show_alert: show_alert_5,
            title: "Bottom",
            alert_class: "text-center rounded-md shadow-lg bg-purple-500 text-white border border-purple-700 p-4",
            icon_type: "success",
            position: "bottom",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-white",
            message_class: "text-purple-300",
            icon_class: "flex justify-center",
            confirm_button_text: "",
            cancel_button_text: "",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "50",
        },
        AlertProps {
            message: "This is in the bottom-left corner.",
            timeout: 1500,
            show_alert: show_alert_6,
            title: "Bottom Left",
            alert_class: "text-center rounded-md shadow-lg bg-orange-500 text-white border border-orange-700 p-4",
            icon_type: "success",
            position: "bottom-left",
            container_class: "",
            cancel_button_class: "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200",
            confirm_button_class: "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200",
            title_class: "text-white",
            message_class: "text-orange-300",
            icon_class: "flex justify-center",
            confirm_button_text: "",
            cancel_button_text: "",
            show_confirm_button: true,
            show_cancel_button: true,
            show_close_button: false,
            on_confirm: Callback::noop(),
            on_cancel: Callback::noop(),
            icon_color: "",
            icon_width: "50",
        },
    ];
    let alert_components: Html = alerts.iter().map(|prop| {

        let callback_fn = {
            let show_alert_state = prop.show_alert.clone();
            Callback::from(move |_| {
                show_alert_state.set(true);
            })
        };

        html! {
            <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                <h2 class="text-xl font-bold mb-2">{prop.title}</h2>
                <pre class="font-mono text-sm text-white p-4 bg-gray-800 rounded-md w-128">
                <code class="">
                    {format!(
                        "<Alert
  title=\"{}\"
  message=\"{}\"
  show_alert={{show_alert.clone()}}
  position=\"{}\"
  container_class=\"{}\"
  title_class=\"{}\"
  message_class=\"{}\"
  icon_type=\"{}\"
  icon_class=\"{}\"
  confirm_button_text=\"{}\"
  cancel_button_text=\"{}\"
  show_confirm_button={}
  show_cancel_button={}
  show_close_button={}
  on_confirm={:?}
  on_cancel={:?}
  icon_color=\"{}\"
  icon_width=\"{}\"
/>",
                        &prop.title,
                        &prop.message,
                        &prop.position,
                        &prop.container_class,
                        &prop.title_class,
                        &prop.message_class,
                        &prop.icon_type,
                        &prop.icon_class,
                        &prop.confirm_button_text,
                        &prop.cancel_button_text,
                        prop.show_confirm_button,
                        prop.show_cancel_button,
                        prop.show_close_button,
                        &prop.on_confirm,
                        &prop.on_cancel,
                        &prop.icon_color,
                        &prop.icon_width
                    )}
                </code>
                </pre>

                <button class="mt-6 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" onclick={callback_fn}>
                    {"Show Alert"}
                </button>
                <Alert
                    ..prop.clone()
                />
            </div>
        }
    }).collect();

    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8  text-white ">{ "Yew Alert" }</h1>
            <div class="grid grid-cols-1 gap-16 mt-6">{ alert_components }</div>
        </div>
    }
}
