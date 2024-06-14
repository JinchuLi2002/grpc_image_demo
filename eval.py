import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime
import os
import numpy as np

# Get the directory of the script
script_dir = os.path.dirname(os.path.abspath(__file__))

# Paths to the log files
client_log_path = os.path.join(
    script_dir, 'grpc_image_client', 'client_log.csv')
server_log_path = os.path.join(
    script_dir, 'grpc_image_server', 'server_log.csv')

# Read the logs
client_log = pd.read_csv(client_log_path, header=None,
                         names=['image_path', 'start_time'])
server_log = pd.read_csv(server_log_path, header=None, names=[
                         'image_path', 'receive_time'])

# Convert timestamps to datetime
client_log['start_time'] = pd.to_datetime(client_log['start_time'])
server_log['receive_time'] = pd.to_datetime(server_log['receive_time'])

# Merge logs on image_path
merged_log = pd.merge(client_log, server_log, on='image_path')

# Calculate transmission time
merged_log['transmission_time'] = (
    merged_log['receive_time'] - merged_log['start_time']).dt.total_seconds() * 1000  # milliseconds

# Get file sizes


def get_file_size(image_path):
    return os.path.getsize(image_path)  # file size in bytes


# Image directory path
image_dir = os.path.join(script_dir, 'images')

merged_log['image_size_kb'] = merged_log['image_path'].apply(
    lambda x: get_file_size(os.path.join(image_dir, os.path.basename(x))) / 1024)  # Convert bytes to KB

# Plot transmission time vs. image size
plt.figure(figsize=(10, 6))
plt.scatter(merged_log['image_size_kb'],
            merged_log['transmission_time'], label='Data points')

# Perform linear regression
x = merged_log['image_size_kb']
y = merged_log['transmission_time']
slope, intercept = np.polyfit(x, y, 1)
fit_line = slope * x + intercept
plt.plot(x, fit_line, color='red', label='Fitting line')

plt.xlabel('File Size (KB)')
plt.ylabel('Transmission Time (ms)')
plt.title('Transmission Time vs. File Size')
plt.legend()
plt.grid(True)
plt.show()

# Print file sizes in KB for verification
for file in os.listdir(image_dir):
    print(
        f'File size of {file} is {get_file_size(os.path.join(image_dir, file)) / 1024} KB')
