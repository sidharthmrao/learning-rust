from pickletools import read_bytes1
import tkinter as tk
import time

def workout(workout_number):
    window = tk.Tk()
    window.configure(background='black')
    window.attributes("-fullscreen", True)
    z = f"""
            WORKOUT #{workout_number}:
            1.) Burpees 45 s
            2.) Squat lunges 45 s
            3.) In and out Plank (open and close) 45 s
            4.) Squat jumps side to side 45 s
            5.) High Knee taps 45 s
        """
    greeting = tk.Label(window, text=z, font=("Times New Roman", 20), foreground="red", background="black")
    greeting.pack()
        
    def ready():
        pass
        
    event_button = tk.Button(window, text="Ready", width="25", height="5", bg="red", fg="black", command=window.destroy)
    event_button.pack()
    
    
    
    window.mainloop()
    
# Every ten minutes, run workout()
while True:
    workout(1)
    starttime = time.time()
    while time.time() < starttime + 600:
        if (600-time.time()-starttime)%1 == 0:
            print(f"{600-(time.time()-starttime).__round__(0)} seconds left")
    