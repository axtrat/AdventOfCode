import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Pattern;

/**
 * Solution
 */
public class Solution {

    public static void main(String[] args) throws FileNotFoundException {
        String fileName = (args.length == 0) ? "input" : args[0];
        List<String> lines = new ArrayList<>();
        try (Scanner sc = new Scanner(new File(fileName + ".txt"))) {
            while (sc.hasNextLine()) {
                lines.add(sc.nextLine());
            }
        }
        MyTimer timer = new MyTimer();
        part1(lines);
        System.out.println(timer);
        timer.restart();
        part2(lines);
        System.out.println(timer);
    }

    private static List<Integer> getInts(String line) {
        Pattern pattern = Pattern.compile("\\d+");
        return pattern.matcher(line).results().map((match) -> Integer.parseInt(match.group())).toList();
    }

    private static void part1(List<String> file) {
        var times = getInts(file.get(0));
        var dinstances = getInts(file.get(1));
        int res = 1;
        for (int i = 0; i < times.size(); i++) {
            int count = 0;
            int time = times.get(i), dinstance = dinstances.get(i);
            for (int j = 1; j <= time; j++) {
                int speed = (time-j) * j;
                if (speed > dinstance) count++;
            }
            res *= count;
        }
        System.out.println(res);
    }

    private static void part2(List<String> file) {
        System.out.println("Not implemented yet");
    }

}