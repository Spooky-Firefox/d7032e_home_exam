// Player.java
// Public fields kept for “quick & dirty”; helpers centralized to reduce clutter.

import java.util.*;

public class Player {
    // --- “Public on purpose” for the exam ---
    public int victoryPoints = 0;
    public int progressPoints = 0;
    public int skillPoints = 0;
    public int commercePoints = 0;
    public int strengthPoints = 0;

    public int tradeRate = 3; // default 3:1 with bank
    public boolean isBot = false;

    // “Flags” for quick state (MARKETPLACE, PARISH, TOLLB, LTS@r,c, 2FOR1_WOOL,
    // STOREHOUSE@r,c, BRIGITTA, SCOUT_NEXT_SETTLEMENT)
    public Set<String> flags = new HashSet<>();

    // Resource pools (coarse, not per region)
    public Map<String, Integer> resources = new HashMap<>();

    // Hand (now real cards)
    public List<Card> hand = new ArrayList<>();

    // Principality: dynamic 2D grid of cards (null = empty)
    public List<List<Card>> principality = new ArrayList<>();

    // Last settlement (for Scout)
    public int lastSettlementRow = -1, lastSettlementCol = -1;

    private final Scanner in = new Scanner(System.in);

    public Player() {
        String[] all = { "Brick", "Grain", "Lumber", "Wool", "Ore", "Gold", "Any" };
        principality = new java.util.ArrayList<>();
        // Start with a 5×5 empty grid (grows as needed)
        for (int r = 0; r < 5; r++) {
            java.util.List<Card> row = new java.util.ArrayList<>();
            for (int c = 0; c < 5; c++)
                row.add(null);
            principality.add(row);
        }
        // (init other maps as new HashMap each; no static/shared refs)
        resources = new java.util.HashMap<>();
        for (String r : all)
            resources.put(r, 0);
    }

    // ------------- I/O (console) -------------
    public void sendMessage(Object m) {
        System.out.println(m);
    }

    public String receiveMessage() {
        System.out.print("> ");
        return in.nextLine();
    }

    // ------------- Grid helpers -------------
    public Card getCard(int r, int c) {
        if (r < 0 || c < 0)
            return null;
        if (r >= principality.size())
            return null;
        List<Card> row = principality.get(r);
        if (row == null || c >= row.size())
            return null;
        return row.get(c);
    }

    public void placeCard(int r, int c, Card card) {
        ensureSize(r, c);
        principality.get(r).set(c, card);
    }

    // Returns the (possibly updated) column where the just-built center card now
    // sits
    public int expandAfterEdgeBuild(int col) {
        int cols = principality.get(0).size();
        // if placed in first column, insert a new column at the far left
        if (col == 0) {
            for (java.util.List<Card> row : principality) {
                row.add(0, null);
            }
            // all existing cards (including the one we just placed) shifted +1
            col += 1;
            if (lastSettlementCol >= 0)
                lastSettlementCol += 1;
        } else if (col == cols - 1) {
            // placed in last column, so append a new rightmost column
            for (java.util.List<Card> row : principality) {
                row.add(null);
            }
            // col stays the same
        }
        return col;
    }

    private void ensureSize(int r, int c) {
        while (principality.size() <= r) {
            ArrayList<Card> row = new ArrayList<>();
            // match width
            int cols = principality.isEmpty() ? 5 : principality.get(0).size();
            for (int i = 0; i < cols; i++)
                row.add(null);
            principality.add(row);
        }
        for (List<Card> row : principality) {
            while (row.size() <= c)
                row.add(null);
        }
    }

    public boolean hasInPrincipality(String name) {
        for (List<Card> row : principality)
            for (Card c : row)
                if (c != null && c.name != null && c.name.equalsIgnoreCase(name))
                    return true;
        return false;
    }

    // Nicely prints the principality with coordinates, plus hand & point summary.
    public String printPrincipality() {
        StringBuilder sb = new StringBuilder();
        int rows = principality.size();
        int cols = principality.isEmpty() ? 0 : principality.get(0).size();

        // Compute column widths based on both title and info lines
        int[] w = new int[cols];
        int minW = 10; // a reasonable minimum so headers fit
        for (int c = 0; c < cols; c++) {
            int m = minW;
            for (int r = 0; r < rows; r++) {
                Card card = getCard(r, c);
                String title = cellTitle(card);
                String info = cellInfo(card);
                m = Math.max(m, title.length());
                m = Math.max(m, info.length());
            }
            w[c] = m;
        }

        // Top header for columns (outside the grid for clarity)
        sb.append("      "); // space for row index column
        for (int c = 0; c < cols; c++) {
            String hdr = "Col " + c;
            sb.append(padRight(hdr, w[c] + 3)); // +3 because inside grid we put spaces and | around content
        }
        sb.append("\n");

        // Top border
        sb.append("    ").append(buildSep(w)).append("\n");

        // Each board row => 2 text lines inside the grid
        for (int r = 0; r < rows; r++) {
            // Title line
            sb.append(String.format("%2d  ", r)); // row index + two spaces
            sb.append("|");
            for (int c = 0; c < cols; c++) {
                String title = cellTitle(getCard(r, c));
                sb.append(" ").append(padRight(title, w[c])).append(" ").append("|");
            }
            sb.append("\n");

            // Info line
            sb.append("    "); // aligns with the top border (no row index on the 2nd line)
            sb.append("|");
            for (int c = 0; c < cols; c++) {
                String info = cellInfo(getCard(r, c));
                sb.append(" ").append(padRight(info, w[c])).append(" ").append("|");
            }
            sb.append("\n");

            // Row separator
            sb.append("    ").append(buildSep(w)).append("\n");
        }

        // Points line
        sb.append("\nPoints: ")
                .append("VP=").append(victoryPoints)
                .append("  CP=").append(commercePoints)
                .append("  SP=").append(skillPoints)
                .append("  FP=").append(strengthPoints)
                .append("  PP=").append(progressPoints)
                .append("\n");

        return sb.toString();
    }

    /**
     * Pretty-print this player's hand with index, cost, and any point values.
     */
    public String printHand() {
        StringBuilder sb = new StringBuilder();
        sb.append("Hand (").append(hand.size()).append("):\n");
        for (int i = 0; i < hand.size(); i++) {
            Card c = hand.get(i);
            if (c == null)
                continue;
            String cost = (c.cost == null || c.cost.isBlank()) ? "-" : c.cost;
            String pts = summarizePoints(c); // same helper you already use in printPrincipality
            sb.append("  [").append(i).append("] ")
                    .append(c.name == null ? "Unknown" : c.name)
                    .append("   {cost: ").append(cost).append("} ")
                    .append(pts.isEmpty() ? "" : pts)
                    .append("\n").append(c.cardText == null ? "" : "\t" + c.cardText + "\n");
        }
        return sb.toString();
    }

    // Build a separator like: +-----------+--------------+---+
    private String buildSep(int[] w) {
        StringBuilder sep = new StringBuilder();
        sep.append("+");
        for (int c = 0; c < w.length; c++) {
            sep.append("-".repeat(w[c] + 2)); // +2 for side spaces inside cells
            sep.append("+");
        }
        return sep.toString();
    }

    // Left-pad to fixed width (for stable '|' alignment)
    private String padRight(String s, int w) {
        if (s == null)
            s = "";
        if (s.length() >= w)
            return s;
        return s + " ".repeat(w - s.length());
    }

    // ---------- helpers used by printPrincipality ----------

    private String cellTitle(Card c) {
        if (c == null)
            return "";
        String title = c.name;
        if (title.equals("Forest"))
            title += " (L):Lumber";
        else if (title.equals("Hill"))
            title += " (B):Brick";
        else if (title.equals("Field"))
            title += " (G):Grain";
        else if (title.equals("Pasture"))
            title += " (W):Wool";
        else if (title.equals("Mountain"))
            title += " (O):Ore";
        else if (title.equals("Gold Field"))
            title += " (A):Gold";
        return title == null ? "Unknown" : title;
    }

    private String cellInfo(Card c) {
        if (c == null)
            return ""; // EMPTY
        // Regions: show dice + stored (0..3)
        if ("Region".equalsIgnoreCase(c.type)) {
            String die = (c.diceRoll <= 0 ? "-" : String.valueOf(c.diceRoll));
            int stored = Math.max(0, Math.min(3, c.regionProduction));
            return "d" + die + "  " + stored + "/3";
        }

        // Common trade ships: "2:1 <Res>"
        String nm = c.name == null ? "" : c.name;
        if (c.type != null && c.type.toLowerCase().contains("trade ship")) {
            if (!nm.equalsIgnoreCase("Large Trade Ship") && nm.endsWith("Ship")) {
                String res = firstWord(nm); // Brick / Grain / etc.
                return "2:1 " + res;
            } else if (nm.equalsIgnoreCase("Large Trade Ship")) {
                return "LTS (left/right swap 2→1)";
            }
        }

        // Boosters: Foundry/Mill/Camp/Factory/Shop (hint text)
        if ("Building".equalsIgnoreCase(c.type) &&
                "Settlement/City Expansions".equalsIgnoreCase(c.placement)) {
            if (nm.endsWith("Foundry"))
                return "Boosts Ore x2 on match";
            if (nm.endsWith("Mill"))
                return "Boosts Grain x2 on match";
            if (nm.endsWith("Camp"))
                return "Boosts Lumber x2 on match";
            if (nm.endsWith("Factory"))
                return "Boosts Brick x2 on match";
            if (nm.endsWith("Shop"))
                return "Boosts Wool x2 on match";
        }

        // Center cards quick hints
        if ("Road".equalsIgnoreCase(nm))
            return "Center";
        if ("Settlement".equalsIgnoreCase(nm))
            return "Center";
        if ("City".equalsIgnoreCase(nm))
            return "Center";

        // Heroes / others: summarize points if any
        String pts = summarizePoints(c);
        if (!pts.isEmpty())
            return pts;

        // Default: show placement/type short
        String pl = c.placement == null ? "" : c.placement;
        String tp = c.type == null ? "" : c.type;
        if (!pl.isEmpty() || !tp.isEmpty())
            return (pl + " " + tp).trim();
        return "";
    }

    // SCORING helper: summarize points on a card like "[VP1 CP2 SP1 FP0 PP0]"
    private String summarizePoints(Card c) {
        // Build a compact points summary like: "[VP1 CP2 SP1 FP0 PP0]"
        int vp = parseIntSafe(c.victoryPoints);
        int cp = parseIntSafe(c.CP);
        int sp = parseIntSafe(c.SP);
        int fp = parseIntSafe(c.FP);
        int pp = parseIntSafe(c.PP);

        StringBuilder t = new StringBuilder();
        if (vp > 0 || cp > 0 || sp > 0 || fp > 0 || pp > 0) {
            t.append("[");
            if (vp > 0)
                t.append("VP").append(vp).append(" ");
            if (cp > 0)
                t.append("CP").append(cp).append(" ");
            if (sp > 0)
                t.append("SP").append(sp).append(" ");
            if (fp > 0)
                t.append("FP").append(fp).append(" ");
            if (pp > 0)
                t.append("PP").append(pp).append(" ");
            if (t.charAt(t.length() - 1) == ' ')
                t.deleteCharAt(t.length() - 1);
            t.append("]");
        }
        return t.toString();
    }

    // Advantage tokens depend on being >= 3 ahead of the opponent.
    public boolean hasTradeTokenAgainst(Player opp) {
        return (this.commercePoints - (opp == null ? 0 : opp.commercePoints)) >= 3;
    }

    public boolean hasStrengthTokenAgainst(Player opp) {
        return (this.strengthPoints - (opp == null ? 0 : opp.strengthPoints)) >= 3;
    }

    // Final score used for win check: base VP + 1 per advantage token against
    // opponent
    public int currentScoreAgainst(Player opp) {
        int score = this.victoryPoints;
        if (hasTradeTokenAgainst(opp))
            score += 1;
        if (hasStrengthTokenAgainst(opp))
            score += 1;
        return score;
    }

    private String firstWord(String s) {
        if (s == null)
            return "";
        String[] toks = s.trim().split("\\s+");
        return toks.length == 0 ? "" : toks[0];
    }

    private int parseIntSafe(String s) {
        if (s == null || s.isBlank())
            return 0;
        try {
            return Integer.parseInt(s.trim());
        } catch (Exception e) {
            return 0;
        }
    }

    // ------------- Resources (per-region, not pooled) -------------

    // Map a resource name to its Region card name
    private String resourceToRegion(String type) {
        if (type == null)
            return null;
        String t = type.trim().toLowerCase();
        switch (t) {
            case "brick":
                return "Hill";
            case "grain":
                return "Field";
            case "lumber":
                return "Forest";
            case "wool":
                return "Pasture";
            case "ore":
                return "Mountain";
            case "gold":
                return "Gold Field";
            case "any":
                return "Any";
            default:
                return null;
        }
    }

    // Collect all Region cards of a given region-name (e.g., "Forest")
    private java.util.List<Card> findRegions(String regionName) {
        java.util.List<Card> list = new java.util.ArrayList<>();
        if (regionName == null)
            return list;
        for (int r = 0; r < principality.size(); r++) {
            java.util.List<Card> row = principality.get(r);
            if (row == null)
                continue;
            for (int c = 0; c < row.size(); c++) {
                Card x = row.get(c);
                if (x != null &&
                        "Region".equalsIgnoreCase(x.type) &&
                        x.name != null &&
                        x.name.equalsIgnoreCase(regionName)) {
                    list.add(x);
                }
            }
        }
        return list;
    }

    // Sum stored resources on all regions (of ANY type)
    public int totalAllResources() {
        int sum = 0;
        for (int r = 0; r < principality.size(); r++) {
            java.util.List<Card> row = principality.get(r);
            if (row == null)
                continue;
            for (int c = 0; c < row.size(); c++) {
                Card x = row.get(c);
                if (x != null && "Region".equalsIgnoreCase(x.type)) {
                    sum += Math.max(0, Math.min(3, x.regionProduction));
                }
            }
        }
        return sum;
    }

    // Count stored resources of a specific resource type across the board
    public int getResourceCount(String type) {
        String regionName = resourceToRegion(type);
        if (regionName == null)
            return 0;
        if ("Any".equals(regionName))
            return totalAllResources();
        int sum = 0;
        for (Card r : findRegions(regionName)) {
            sum += Math.max(0, Math.min(3, r.regionProduction));
        }
        return sum;
    }

    // Gain 1 resource of a type: add to the matching region with the LOWEST stock
    // (<3)
    // If "Any", ask the player which resource to take.
    public void gainResource(String type) {
        String t = type;
        if (t == null || t.equalsIgnoreCase("Any")) {
            sendMessage("PROMPT: Choose resource to gain (Brick/Grain/Lumber/Wool/Ore/Gold):");
            t = receiveMessage();
        }
        String regionName = resourceToRegion(t);
        if (regionName == null || "Any".equals(regionName)) {
            sendMessage("Unknown resource '" + t + "'. Ignored.");
            return;
        }

        java.util.List<Card> regs = findRegions(regionName);
        if (regs.isEmpty()) {
            sendMessage("No region for resource " + t + " is present.");
            return;
        }

        // pick lowest stored (<3); tie -> first in board order
        Card best = null;
        int bestVal = Integer.MAX_VALUE;
        for (Card r : regs) {
            int v = Math.max(0, Math.min(3, r.regionProduction));
            if (v < bestVal) {
                bestVal = v;
                best = r;
            }
        }
        if (best != null && best.regionProduction < 3) {
            best.regionProduction += 1;
            // Optional: feedback
            // sendMessage("Gained 1 " + t + " on " + regionName + " (" +
            // (best.regionProduction) + "/3)");
        } else {
            sendMessage("No storage space on any " + regionName + " (already 3/3).");
        }
    }

    // Remove N resources of a type: repeatedly remove from the region with the
    // HIGHEST stock (>0)
    // Returns true if all could be removed, false otherwise (removes as many as
    // possible).
    public boolean removeResource(String type, int n) {
        if (n <= 0)
            return true;
        String regionName = resourceToRegion(type);
        if (regionName == null || "Any".equals(regionName))
            return false;

        java.util.List<Card> regs = findRegions(regionName);
        if (regs.isEmpty())
            return false;

        int removed = 0;
        while (removed < n) {
            // find highest stocked region (>0)
            Card best = null;
            int bestVal = -1;
            for (Card r : regs) {
                int v = Math.max(0, Math.min(3, r.regionProduction));
                if (v > bestVal) {
                    bestVal = v;
                    best = r;
                }
            }
            if (best == null || bestVal <= 0)
                break; // no more to remove
            best.regionProduction -= 1;
            removed++;
        }
        return removed == n;
    }

    // Set total stored resources for a type by redistributing across its regions.
    // If n <= 0, zero all matching regions.
    // If increasing, fill lowest first; if decreasing, remove from highest first.
    public void setResourceCount(String type, int n) {
        String regionName = resourceToRegion(type);
        if (regionName == null || "Any".equals(regionName))
            return;

        java.util.List<Card> regs = findRegions(regionName);
        if (regs.isEmpty())
            return;

        // clamp desired total between 0 and regions*3
        int maxTotal = regs.size() * 3;
        int want = Math.max(0, Math.min(maxTotal, n));

        // current total
        int cur = 0;
        for (Card r : regs) {
            r.regionProduction = Math.max(0, Math.min(3, r.regionProduction)); // sanitize
            cur += r.regionProduction;
        }
        if (cur == want)
            return;

        if (cur < want) {
            // add (want - cur) by filling lowest first
            int need = want - cur;
            while (need > 0) {
                Card best = null;
                int bestVal = Integer.MAX_VALUE;
                for (Card r : regs) {
                    int v = r.regionProduction;
                    if (v < 3 && v < bestVal) {
                        bestVal = v;
                        best = r;
                    }
                }
                if (best == null || best.regionProduction >= 3)
                    break;
                best.regionProduction += 1;
                need--;
            }
        } else {
            // remove (cur - want) by draining highest first
            int drop = cur - want;
            while (drop > 0) {
                Card best = null;
                int bestVal = -1;
                for (Card r : regs) {
                    int v = r.regionProduction;
                    if (v > bestVal) {
                        bestVal = v;
                        best = r;
                    }
                }
                if (best == null || best.regionProduction <= 0)
                    break;
                best.regionProduction -= 1;
                drop--;
            }
        }
    }

    // ------------- Hand -------------
    public int handSize() {
        return hand.size();
    }

    public void addToHand(Card c) {
        hand.add(c);
    }

    public Card removeFromHandByName(String nm) {
        for (int i = 0; i < hand.size(); i++) {
            Card c = hand.get(i);
            if (c != null && c.name != null && c.name.equalsIgnoreCase(nm)) {
                return hand.remove(i);
            }
        }
        return null;
    }

    // ------------- Cheap prompts used by Server -------------
    public String chooseResource() {
        sendMessage("PROMPT: Choose resource:");
        return receiveMessage();
    }
}