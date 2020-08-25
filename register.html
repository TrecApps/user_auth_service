
<!DOCTYPE html>
<html>
	<head>
		<meta charset="UTF-8">
		<title>Trec-Apps Register</title>
		<link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css" integrity="sha384-MCw98/SFnGE8fJT3GXwEOngsV7Zt27NXFoaoApmYm81iuXoPkFOJwJ8ERdknLPMO" crossorigin="anonymous">
    	<script src="https://code.jquery.com/jquery-3.3.1.slim.min.js" integrity="sha384-q8i/X+965DzO0rT7abK41JStQIAqVgRVzpbzo5smXKp4YfRvH+8abtTE1Pi6jizo" crossorigin="anonymous"></script>
    	<script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.3/umd/popper.min.js" integrity="sha384-ZMP7rVo3mIykV+2+9J3UJ46jBk0WLaUAdn689aCwoqbBJiSnjAK/l8WvCWPIPm49" crossorigin="anonymous"></script>
		<script src="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/js/bootstrap.min.js" integrity="sha384-ChfqqxuZUCnJSK3+MXmPNIyE6ZbWh2IMqE241rYiqJxyMiZ6OW/JmZQ5stwEULTy" crossorigin="anonymous"></script>
	</head>
	<body>
	
		<script type="application/JavaScript">
		
		var username;
		
		var userNameReady = false;
		var passwordsMatch = false;
		
		function checkUserName() {
			let xhr = (new XMLHttpRequest() || new ActiveXObject("Microsoft.HTTPRequest"));
			
			let initUrl = window.location.href;
			
			username = document.getElementById("userField").value;
			
			initUrl += '/UserExists/' + username;
			
			xhr.onreadystatechange = function() {
				
				if(this.readyState === 4 && this.status == 200){
					let result = this.responseText;
					
					console.log("Result: "+ result);
					
					result = result.toUpperCase();
					
					let userLabel = document.getElementById("userField");
					
					if(result == "TRUE") {
						userLabel.className = "bg-danger";
						userNameReady = false;
					} else {
						userLabel.className = "bg-info";
						if(username.length > 5 && username.length < 31) {
							userNameReady = true;
						} else {
							userNameReady = false;
						}
					}
				}
			}
			initUrl = initUrl.replace("/**", "");
			initUrl = initUrl.replace("/NewUser", "");
			initUrl = initUrl.replace("/failed", "");
			console.log("initUrl=", initUrl);
			xhr.open("GET", initUrl)
			xhr.send();
		}
		
		function comparePasswords()	{
			let p1 = document.getElementById("password1").value;
			let p2 = document.getElementById("password2").value;
			
			let passBanner = document.getElementById("passWarning");
			
			if(p1 === p2 && p1.length > 8) {
				passwordsMatch = true;
				passBanner.hidden = true;
			} else {
				passwordsMatch = false;
				passBanner.hidden = false;
			}
		}
		
		function validateSubmit() {
			return userNameReady && passwordsMatch;
		}
		
		</script>
	
		<div class="container">
			<div id="messageJumbo" class="jumbotron">${message}
			</div>
			
			<form width="100%" class="form" id="Userform" action="/NewUser" method="post" onsubmit="return validateSubmit();">
				<label>User-name (Between 6 and 30 characters):</label>
				<input required class="bg-info" id="userField" name="username" type="text" oninput="checkUserName()">
				<br>
				<label>Email:</label>
				<input required name="email" type="email">
				<br>
				<label>Password:</label>
				<input required name="password" type="password" id="password1" oninput="comparePasswords()">
				<br>
				<label>Confirm Password:</label>
				<input required type="password" id="password2" oninput="comparePasswords()">
				<h5 class="bg-danger" id="passWarning" hidden> Passwords must match and be 8+ characters in length!</h5>
				<input type="submit" class="btn btn=submit">
			</form>
			
			
		</div>
	</body>
</html>


