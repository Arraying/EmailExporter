import sys
import mailbox
import email.utils


def main():
    if len(sys.argv) <= 1:
        print("Please provide the path to the file as the first argument")
        return
    path = " ".join(sys.argv[1:])
    mbox = mailbox.mbox(path)
    mbox.lock()
    addrs = dict()
    try:
        for mail in mbox:
            from_string = str(mail["From"]).lower()
            [_, addr] = email.utils.parseaddr(from_string)
            parts = addr.split("@")
            # Some CPM names don't work
            if len(parts) < 2:
                continue
            username = parts[0]
            domain = parts[1]
            users = addrs.get(domain, set())
            users.add(username)
            addrs[domain] = users
    finally:
        mbox.unlock()
    for domain, users in addrs.items():
        print(domain)
        for user in users:
            print(f"\t{user}@{domain}")


if __name__ == "__main__":
    main()