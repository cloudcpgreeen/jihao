import smtplib
import os
import sys
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

def main():
    msg_path = sys.argv[1] if len(sys.argv) > 1 else "message/final.md"

    with open(msg_path) as f:
        body = f.read()

    mime = MIMEMultipart()
    mime["From"] = os.environ["SMTP_USER"]
    mime["To"] = os.environ["MAIL_TO"]
    mime["Subject"] = "爱的港湾"

    mime.attach(MIMEText(body, "plain", "utf-8"))

    with smtplib.SMTP_SSL(os.environ["SMTP_HOST"], int(os.environ["SMTP_PORT"])) as s:
        s.login(os.environ["SMTP_USER"], os.environ["SMTP_PASSWORD"])
        s.send_message(mime)

    print("Sent.")

if __name__ == "__main__":
    main()
