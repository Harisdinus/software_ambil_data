
const servo = {
  degree: 0,
  vibrate: false,
}

const server = Bun.serve({
  fetch(req) {
    const url = new URL(req.url);

    if (req.method == "GET") {
      return handle_get(req, url);
    }

    if (req.method == "POST") {
      return handle_post(req, url);
    }

    const error = `Not Found: ${req.method} ${url.pathname}`;
    console.error("Error:", error);
    return new Response(error, { status: 404 });
  },
})

async function handle_get(req, url) {
  console.log(req.method,url.pathname);

  if (url.pathname == "/geterin") {
    servo.vibrate = !servo.vibrate
  }

  console.log("Vibrate:",servo.vibrate)

  return new Response(`OK\n{"topic":"Menengo","vibrate":${servo.vibrate}}`);
}

async function handle_post(req, url) {
  const data = await req.json();

  console.log(req.method,url.pathname);
  console.log(JSON.stringify(data,null,2));

  if (data.perintah == "servo_up") {
    servo.degree += data.value;
  }

  if (data.perintah == "servo_down") {
    servo.degree -= data.value;
  }

  if (data.perintah == "servo_reset") {
    servo.degree = 0;
  }

  return new Response(`OK\n{"topic":"Menengo","degree":${servo.degree}}`);
}

console.log("listening",server.url.href)

