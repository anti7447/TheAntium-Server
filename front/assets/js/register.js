async function register() {
  const input_login = document.getElementById("register_login");
  const input_name = document.getElementById("register_name");
  const input_password = document.getElementById("register_password");

  const login = input_login.value;
  const name = input_name.value;
  const password = input_password.value;

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
    if (response.status == 409) {
      alert(`Логин занят`);
      return;
    }
    alert(`Ошибка: ${response.status}`);
    return;
  }

  window.location.href = "/";
}
