async function register() {
  const input_login = document.getElementById("register_login");
  const input_name = document.getElementById("register_name");
  const input_password = document.getElementById("register_password");
  const button = document.getElementById("register_button");

  const login = input_login.value;
  const name = input_name.value;
  const password = input_password.value;

  button.disabled = true;

  const response = await fetch("/api/v1/users", {
    method: "POST",
    headers: {
      "Content-Type": "application/json", // Указываем формат данных
      // 'Content-Type': 'application/x-www-form-urlencoded', // Для обычных форм
    },
    body: JSON.stringify({
      tag: login,
      username: name,
      password: password,
    }),
  });

  if (!response.ok) {
    button.disabled = false;
    if (response.status == 409) {
      alert(`Логин занят`);
      return;
    }
    alert(`Ошибка: ${response.status}`);
    return;
  }

  window.location.href = "/login";
  button.disabled = false;
}

async function signin() {
  const input_login = document.getElementById("signin_login");
  const input_password = document.getElementById("signin_password");
  const button = document.getElementById("signin_button");

  const login = input_login.value;
  const password = input_password.value;

  console.log(login, password);
  button.disabled = true;

  const response = await fetch("/auth/login", {
    method: "POST",
    headers: {
      "Content-Type": "application/json", // Указываем формат данных
      // 'Content-Type': 'application/x-www-form-urlencoded', // Для обычных форм
    },
    credentials: "include",
    body: JSON.stringify({
      tag: login,
      password: password,
    }),
  });

  if (!response.ok) {
    button.disabled = false;
    if (response.status == 409) {
      alert(`Что-то не так, спроси в тг :(`);
      return;
    }
    alert(`Ошибка: ${response.status}`);
    return;
  }

  // alert(response.status);
  window.location.href = "/user";
  button.disabled = false;
}
