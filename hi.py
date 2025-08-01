import sys, time
data = sys.stdin.read()
#time.sleep(5)  # Simulate slow CGI
print(f"hello {data.strip()}")
