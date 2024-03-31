# Instalacion

1. Clonar repositorio

```bash
git clone https://github.com/b-munar/auth.git
```

2. Copiar el .env.template y pegarlo en un .env

3. 

```bash
docker compose build
docker compose up
```


El servicio de autenticación, este hace registro y login de usuario, tambien su funcion principal es la de autenticar.

## 1. Creación de usuarios

Crea un usuario con los datos brindados, el email del usuario debe ser único.

<table>
<tr>
<td> Método </td>
<td> POST </td>
</tr>
<tr>
<td> Ruta </td>
<td> <strong>localhost:6050/auth/register</strong> </td>
</tr>
<tr>
<td> Parámetros </td>
<td> N/A </td>
</tr>
<tr>
<td> Encabezados </td>
<td>N/A</td>
</tr>
<tr>
<td> Cuerpo </td>
<td>

```json
{
"email": "email@email.com",
"password": "password",
"id": "29a3ad78-6d3c-46e3-9c42-857ca3ec5220"
}
```
</td>
</tr>
</table>

### Respuestas

<table>
<tr>
<th> Código </th>
<th> Descripción </th>
<th> Cuerpo </th>
</tr>
<tbody>
<td> 201 </td>
<td>En el caso que el usuario se haya creado con éxito.</td>
<td>

```json
{
  "auth": {
    "email": "email@email.com",
    "token": "eyJ0eXA..."
  }
}
```
</td>
</tr>
</tbody>
</table>


## 2. Login de usuarios

Inicia sesion de usuario con los datos brindados

<table>
<tr>
<td> Método </td>
<td> POST </td>
</tr>
<tr>
<td> Ruta </td>
<td> <strong>localhost:6050/auth/login</strong> </td>
</tr>
<tr>
<td> Parámetros </td>
<td> N/A </td>
</tr>
<tr>
<td> Encabezados </td>
<td>N/A</td>
</tr>
<tr>
<td> Cuerpo </td>
<td>

```json
{
"email": "email@email.com",
"password": "password"
}
```
</td>
</tr>
</table>

### Respuestas

<table>
<tr>
<th> Código </th>
<th> Descripción </th>
<th> Cuerpo </th>
</tr>
<tbody>
<td> 202 </td>
<td>En el caso que el usuario hizo un login exitoso.</td>
<td>

```json
{
  "auth": {
    "email": "email@email.com",
    "token": "eyJ0eXA..."
  }
}
```
</td>
</tr>
</tbody>
</table>
