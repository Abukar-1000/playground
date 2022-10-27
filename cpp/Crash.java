package cpp;

import java.util.*;
import java.util.regex.MatchResult;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.io.*;
import java.util.stream.Stream;

class WantedProcess {
    public String name;
    public Long PID;
    public WantedProcess(String name,Long ID){
        this.name = name;
        this.PID = ID;
    }
} 

public class Crash extends Thread{
    public ArrayList<String> processes = new ArrayList<String>();
    public Stream<ProcessHandle> allPrcesses;
    public ArrayList<Long> invalidProcess = new ArrayList<Long>(Arrays.asList(

    ));
    public Crash(){
        try {
            String process;
            // getRuntime: Returns the runtime object associated with the current Java application.
            // exec: Executes the specified string command in a separate process.
            Process p = Runtime.getRuntime().exec("tasklist");
            BufferedReader input = new BufferedReader(new InputStreamReader(p.getInputStream()));

            this.allPrcesses = ProcessHandle.allProcesses();
            while ((process = input.readLine()) != null) {
                System.out.println(process); // <-- Print all Process here line
                                                // by line
                
            }
            input.close();
        } catch (Exception err) {
            err.printStackTrace();
        }
    }
    public void listen(ArrayList<String> targets){
        
        while (true){

        }
    }
    public void show(ProcessHandle process){
        Pattern pat = Pattern.compile("Discord.exe", Pattern.CASE_INSENSITIVE);
        System.out.println("\n\n");
        System.out.println(process.info().toString());
        Matcher matches = pat.matcher(process.info().toString());
        if (matches.find()){
            System.out.println("\n______________________________________________________________________________________________________________________________________________________________________________________________________________________________");
            // System.out.println(matches.group());
            System.out.println("found\n\n\n");
            System.out.println("Discord ID is: " + process.pid());
            this.killProc(process.pid());

        }
    }
    public void killProc(Long ProcessID){
        try {
            // discord process 
           Optional<ProcessHandle> discordHandle = ProcessHandle.of(ProcessID);
           discordHandle.ifPresent(handle -> handle.destroy());

        } catch (Exception err){
            err.printStackTrace();
        }
    } 

    public static void main(String[] args) {
        Crash app = new Crash();
        Long discord = 14788l;
        try {
            // discord process 
           Optional<ProcessHandle> discordHandle = ProcessHandle.of(discord);
           discordHandle.ifPresent(handle -> handle.destroy());

            app.allPrcesses.forEach(process -> app.show(process));
        } catch (Exception err){
            err.printStackTrace();
        }
    }
}