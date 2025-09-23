// Card.java
// Quick & dirty, Basic-set only, refactored to reduce duplication

import com.google.gson.JsonArray;
import com.google.gson.JsonElement;
import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

import java.io.FileReader;
import java.io.IOException;
import java.util.*;

public class Card implements Comparable<Card> {

    // ---------- Public fields (keep simple for the take-home) ----------
    public String name, theme, type, placement, cost, oneOf;
    public String victoryPoints, CP, SP, FP, PP, LP, KP, cardText;
    public String germanName, Requires, protectionOrRemoval;

    // Regions track “stored” resources by rotating; here we model it as an int
    // (0..3)
    public int regionProduction = 0;
    // Regions use production die faces (1..6). 0 means “not a region” / unassigned.
    public int diceRoll = 0;

    // ---------- Global piles for the Basic set ----------
    public static Vector<Card> regions = new Vector<>();
    public static Vector<Card> roads = new Vector<>();
    public static Vector<Card> settlements = new Vector<>();
    public static Vector<Card> cities = new Vector<>();
    public static Vector<Card> events = new Vector<>();
    public static Vector<Card> drawStack1 = new Vector<>();
    public static Vector<Card> drawStack2 = new Vector<>();
    public static Vector<Card> drawStack3 = new Vector<>();
    public static Vector<Card> drawStack4 = new Vector<>();

    // ---------- Construction ----------
    public Card() {
    }

    public Card(String name, String theme, String type,
            String germanName, String placement,
            String oneOf, String cost,
            String victoryPoints, String CP, String SP, String FP,
            String PP, String LP, String KP, String Requires,
            String cardText, String protectionOrRemoval) {
        this.name = name;
        this.theme = theme;
        this.type = type;
        this.germanName = germanName;
        this.placement = placement;
        this.oneOf = oneOf;
        this.cost = cost;
        this.victoryPoints = victoryPoints;
        this.CP = CP;
        this.SP = SP;
        this.FP = FP;
        this.PP = PP;
        this.LP = LP;
        this.KP = KP;
        this.Requires = Requires;
        this.cardText = cardText;
        this.protectionOrRemoval = protectionOrRemoval;
    }

    @Override
    public String toString() {
        return name;
    }

    @Override
    public int compareTo(Card o) {
        return this.name.compareToIgnoreCase(o.name);
    }

    // ---------- Tiny helpers (kept local so the file stays self-contained)
    // ----------
    static boolean nmEquals(String a, String b) {
        return a != null && a.equalsIgnoreCase(b);
    }

    static boolean nmAt(Card c, String a, String b) {
        if (c == null || c.name == null)
            return false;
        String n = c.name;
        return n.equalsIgnoreCase(a) || n.equalsIgnoreCase(b);
    }

    static int asInt(String s, int def) {
        try {
            if (s == null)
                return def;
            return Integer.parseInt(s.trim());
        } catch (Exception e) {
            return def;
        }
    }

    static String gs(JsonObject o, String k) {
        if (!o.has(k))
            return null;
        JsonElement e = o.get(k);
        return (e == null || e.isJsonNull()) ? null : e.getAsString();
    }

    static int gi(JsonObject o, String k, int def) {
        if (!o.has(k))
            return def;
        try {
            return o.get(k).getAsInt();
        } catch (Exception e) {
            return def;
        }
    }

    // Pop first card by name (case-insensitive) from a vector
    // BUG: when initializing principality we don't seem to remove the item so it
    // overwrites...
    public static Card popCardByName(Vector<Card> cards, String name) {
        if (cards == null || name == null)
            return null;
        String target = name.trim();
        for (int i = 0; i < cards.size(); i++) {
            Card c = cards.get(i);
            if (c != null && c.name != null && c.name.trim().equalsIgnoreCase(target)) {
                // Remove-by-index guarantees we return a unique instance and it’s gone from the
                // deck
                return cards.remove(i);
            }
        }
        return null; // not found
    }

    // Extract all cards whose public String field `attribute` equals `value`
    public static Vector<Card> extractCardsByAttribute(Vector<Card> cards, String attribute, String value) {
        Vector<Card> out = new Vector<>();
        try {
            java.lang.reflect.Field f = Card.class.getField(attribute);
            for (int i = cards.size() - 1; i >= 0; i--) {
                Card c = cards.get(i);
                Object v = f.get(c);
                if (v != null && String.valueOf(v).equalsIgnoreCase(value)) {
                    out.add(0, cards.remove(i));
                }
            }
        } catch (Exception ignored) {
        }
        return out;
    }

    // ---------- Loading ONLY the Basic set into piles ----------
    public static void loadBasicCards(String jsonPath) throws IOException {
        Vector<Card> allBasic = new Vector<>();

        try (FileReader fr = new FileReader(jsonPath)) {
            JsonElement root = JsonParser.parseReader(fr);
            if (!root.isJsonArray())
                throw new IOException("cards.json: expected top-level array");
            JsonArray arr = root.getAsJsonArray();

            for (JsonElement el : arr) {
                if (!el.isJsonObject())
                    continue;
                JsonObject o = el.getAsJsonObject();
                String theme = gs(o, "theme");
                if (theme == null || !theme.toLowerCase().contains("basic"))
                    continue; // Introductory only

                int number = gi(o, "number", 1);
                for (int i = 0; i < number; i++) {
                    Card proto = new Card(
                            gs(o, "name"), theme, gs(o, "type"),
                            gs(o, "germanName"), gs(o, "placement"),
                            gs(o, "oneOf"), gs(o, "cost"),
                            gs(o, "victoryPoints"), gs(o, "CP"), gs(o, "SP"), gs(o, "FP"),
                            gs(o, "PP"), gs(o, "LP"), gs(o, "KP"), gs(o, "Requires"),
                            gs(o, "cardText"), gs(o, "protectionOrRemoval"));
                    allBasic.add(proto);
                }
            }
        }

        // Split into piles we care about
        // Center cards
        roads = extractCardsByAttribute(allBasic, "name", "Road");
        settlements = extractCardsByAttribute(allBasic, "name", "Settlement");
        cities = extractCardsByAttribute(allBasic, "name", "City");

        // Regions: "type" == "Region"
        regions = extractCardsByAttribute(allBasic, "type", "Region");

        // Events
        events = extractCardsByAttribute(allBasic, "placement", "Event");
        // Place Yule 4th from bottom per cheat sheet
        Card yule = popCardByName(events, "Yule");
        Collections.shuffle(events);
        if (yule != null && events.size() >= 3) {
            events.add(Math.max(0, events.size() - 3), yule);
        }

        // Remaining “draw stack” cards (action/expansion/units)
        Collections.shuffle(allBasic);
        int stackSize = 9; // Intro game
        drawStack1 = new Vector<>(allBasic.subList(0, Math.min(stackSize, allBasic.size())));
        drawStack2 = new Vector<>(allBasic.subList(Math.min(stackSize, allBasic.size()),
                Math.min(2 * stackSize, allBasic.size())));
        drawStack3 = new Vector<>(allBasic.subList(Math.min(2 * stackSize, allBasic.size()),
                Math.min(3 * stackSize, allBasic.size())));
        drawStack4 = new Vector<>(allBasic.subList(Math.min(3 * stackSize, allBasic.size()),
                Math.min(4 * stackSize, allBasic.size())));
    }

    // ---------- Placement validations (ugly but centralized) ----------
    private static boolean isAboveOrBelowSettlementOrCity(Player p, int row, int col) {
        // Inner ring: ±1 from center settlement/city
        Card up1 = p.getCard(row - 1, col);
        Card down1 = p.getCard(row + 1, col);
        if (nmAt(up1, "Settlement", "City"))
            return true;
        if (nmAt(down1, "Settlement", "City"))
            return true;

        // Outer ring allowed *only* if the inner slot is already filled (fill inner
        // first)
        Card up2 = p.getCard(row - 2, col);
        Card down2 = p.getCard(row + 2, col);
        boolean outerOK = ((nmAt(up2, "City", "City") || nmAt(up2, "Settlement", "Settlement")) && up1 != null) ||
                ((nmAt(down2, "City", "City") || nmAt(down2, "Settlement", "Settlement")) && down1 != null);

        return outerOK;
    }

    private static boolean isCenterSlot(int row) {
        // By convention: inner/outer rows hold expansions & regions; middle row is
        // center (roads/settlements/cities)
        return row % 2 == 0; // 0,2,4,... => center columns; we use row 2 initially as center
    }

    // Determine if region name matches what a booster affects
    public static boolean buildingBoostsRegion(String buildingName, String regionName) {
        if (buildingName == null || regionName == null)
            return false;
        if (buildingName.equalsIgnoreCase("Iron Foundry") && regionName.equalsIgnoreCase("Mountain"))
            return true;
        if (buildingName.equalsIgnoreCase("Grain Mill") && regionName.equalsIgnoreCase("Field"))
            return true;
        if (buildingName.equalsIgnoreCase("Lumber Camp") && regionName.equalsIgnoreCase("Forest"))
            return true;
        if (buildingName.equalsIgnoreCase("Brick Factory") && regionName.equalsIgnoreCase("Hill"))
            return true;
        if (buildingName.equalsIgnoreCase("Weaver’s Shop") && regionName.equalsIgnoreCase("Pasture"))
            return true;
        if (buildingName.equalsIgnoreCase("Weaver's Shop") && regionName.equalsIgnoreCase("Pasture"))
            return true; // ascii
        return false;
    }

    // Place the two diagonal regions after a new settlement (ugly prompt kept here)
    private void placeTwoDiagonalRegions(Player active, int row, int col) {
        // Decide which side is the “open side” (the side without a road)
        int colMod = (active.getCard(row, col - 1) == null) ? -1 : 1;
        int sideCol = col + colMod;

        // Draw or choose 2 regions
        Card first, second;

        if (active.flags.contains("SCOUT_NEXT_SETTLEMENT")) {
            // SCOUT: let player pick two specific regions from the region stack by name or
            // index
            active.sendMessage("PROMPT: SCOUT - Choose first region (name or index):");
            String s1 = active.receiveMessage();
            first = pickRegionFromStackByNameOrIndex(s1);
            if (first == null) {
                // fallback to top
                first = Card.regions.isEmpty() ? null : Card.regions.remove(0);
            }

            active.sendMessage("PROMPT: SCOUT - Choose second region (name or index):");
            String s2 = active.receiveMessage();
            second = pickRegionFromStackByNameOrIndex(s2);
            if (second == null) {
                second = Card.regions.isEmpty() ? null : Card.regions.remove(0);
            }

            if (first == null || second == null) {
                active.sendMessage("SCOUT: Region stack exhausted.");
                // still clear the flag to avoid leaking it
                active.flags.remove("SCOUT_NEXT_SETTLEMENT");
                return;
            }
        } else {
            // normal: take top two
            if (Card.regions.size() < 2) {
                active.sendMessage("Region stack does not have two cards.");
                return;
            }
            first = Card.regions.remove(0);
            second = Card.regions.remove(0);
        }

        // Tell the player which two we drew/selected
        active.sendMessage("New settlement regions drawn/selected:");
        active.sendMessage("  1) " + first.name + "   2) " + second.name);

        // Ask where to put the first one (top/bottom), second goes to the other
        active.sendMessage("PROMPT: Place FIRST region on " + (colMod == -1 ? "LEFT" : "RIGHT")
                + " side: TOP or BOTTOM? (T/B)");
        String choice = active.receiveMessage();
        boolean top = choice != null && choice.trim().toUpperCase().startsWith("T");
        row = 2; // center row
        int topRow = row - 1;
        int bottomRow = row + 1;

        if (top) {
            active.placeCard(topRow, sideCol, first);
            active.placeCard(bottomRow, sideCol, second);
        } else {
            active.placeCard(topRow, sideCol, second);
            active.placeCard(bottomRow, sideCol, first);
        }

        // SCOUT benefit is consumed now; clear the flag
        active.flags.remove("SCOUT_NEXT_SETTLEMENT");
    }

    // Helper: choose region by name or index from Card.regions
    private Card pickRegionFromStackByNameOrIndex(String spec) {
        if (spec == null || spec.isBlank())
            return null;
        spec = spec.trim();
        // try index
        try {
            int idx = Integer.parseInt(spec);
            if (idx >= 0 && idx < Card.regions.size()) {
                return Card.regions.remove(idx);
            }
        } catch (Exception ignored) {
        }
        // try by name (first match)
        for (int i = 0; i < Card.regions.size(); i++) {
            Card c = Card.regions.get(i);
            if (c != null && c.name != null && c.name.equalsIgnoreCase(spec)) {
                return Card.regions.remove(i);
            }
        }
        return null;
    }

    // ---------- Main effect / placement entry ----------
    // Returns true if placed/applied; false if illegal placement
    public boolean applyEffect(Player active, Player other, int row, int col) {
        String nm = (name == null ? "" : name);
        System.out.println("ApplyEffect: " + nm + " at (" + row + "," + col + ")");
        // 0) Early validation for occupied slot
        if (active.getCard(row, col) != null) {
            active.sendMessage("That space is occupied.");
            return false;
        }

        // 1) Center cards: Road / Settlement / City
        if (nmEquals(nm, "Road") || nmEquals(nm, "Settlement") || nmEquals(nm, "City")) {
            if (!isCenterSlot(row)) {
                active.sendMessage("Roads/Settlements/Cities must go in the center row(s).");
                return false;
            }

            if (nmEquals(nm, "Road")) {
                // Just allow anywhere center that touches a center card or extends line
                // (Keep it permissive/ugly)
                active.placeCard(row, col, this);
                active.sendMessage("Built a Road.");
                // Expand board if we built at an edge
                active.expandAfterEdgeBuild(col);
                return true;
            }

            if (nmEquals(nm, "Settlement")) {
                // Simplified: must be next to a Road (left or right) and empty here
                Card L1 = active.getCard(row, col - 1);
                Card R1 = active.getCard(row, col + 1);
                boolean hasRoad = (L1 != null && nmEquals(L1.name, "Road"))
                        || (R1 != null && nmEquals(R1.name, "Road"));
                if (!hasRoad) {
                    active.sendMessage("Settlement must be placed next to a Road.");
                    return false;
                }
                active.placeCard(row, col, this);
                active.victoryPoints += 1;

                // Expand and capture the updated column
                col = active.expandAfterEdgeBuild(col);

                // Now place diagonals using the correct, updated col
                placeTwoDiagonalRegions(active, row, col);

                active.lastSettlementRow = row;
                active.lastSettlementCol = col;
                return true;
            }

            if (nmEquals(nm, "City")) {
                // Must be on top of an existing settlement in the same slot (same row,col)
                // For ugly simplicity: allow upgrading if a Settlement exists exactly here
                Card under = active.getCard(row, col);
                if (under == null || !nmEquals(under.name, "Settlement")) {
                    active.sendMessage("City must be placed on top of an existing Settlement (same slot).");
                    return false;
                }
                active.placeCard(row, col, this);
                active.victoryPoints += 1; // city is 2VP total; settlement vp ignored here, we just add +1
                return true;
            }
        }

        // 2) Regions: allow only in region rows (not center); we set default
        // production=1
        if ("Region".equalsIgnoreCase(type)) {
            if (isCenterSlot(row)) {
                active.sendMessage("Regions must be placed above/below the center row.");
                return false;
            }
            if (regionProduction <= 0)
                regionProduction = 1;
            active.placeCard(row, col, this);
            return true;
        }

        // 3) Settlement/City Expansions (Buildings & Units)
        if (placement != null && placement.equalsIgnoreCase("Settlement/city")) {
            if (!isAboveOrBelowSettlementOrCity(active, row, col)) {
                active.sendMessage("Expansion must be above/below a Settlement or City (fill inner ring first).");
                return false;
            }
            // one-of check (simple)
            if (oneOf != null && oneOf.trim().equalsIgnoreCase("1x")) {
                if (active.hasInPrincipality(nm)) {
                    active.sendMessage("You may only have one '" + nm + "' in your principality.");
                    return false;
                }
            }
            System.out.println("Passed placement checks for " + nm);
            // Buildings that “double” adjacent regions when the number hits (enforced
            // during production)
            if ("Building".equalsIgnoreCase(type)) {
                // Just place it. Production phase will check adjacency and apply +1 increment
                // (cap 3).
                active.placeCard(row, col, this);
                System.out.println("Contained Building");
                if (nmEquals(nm, "Abbey")) {
                    active.progressPoints += 1;
                } else if (nmEquals(nm, "Marketplace")) {
                    active.flags.add("MARKETPLACE");
                } else if (nmEquals(nm, "Parish Hall")) {
                    active.flags.add("PARISH");
                } else if (nmEquals(nm, "Storehouse")) {
                    active.flags.add("STOREHOUSE@" + row + "," + col);
                } else if (nmEquals(nm, "Toll Bridge")) {
                    active.flags.add("TOLLB");
                }
                return true;
            }

            // Units
            if (type.contains("Unit")) {
                System.out.println("Contained UNIT!!!!!");
                // Large Trade Ship: adjacency 2-for-1 between L/R regions (handled in Server)
                if (nmEquals(nm, "Large Trade Ship")) {
                    active.placeCard(row, col, this);
                    active.flags.add("LTS@" + row + "," + col);
                    return true;
                }
                // “Common” trade ships: 2:1 bank for specific resource (handled in Server)
                if (nm.toLowerCase().endsWith(" ship")) {
                    active.placeCard(row, col, this);
                    String res = nm.split("\\s+")[0]; // Brick/Gold/Grain/Lumber/Ore/Wool
                    active.flags.add("2FOR1_" + res.toUpperCase());
                    return true;
                }

                // Heroes: just add SP/FP/CP/etc.
                int sp = asInt(SP, 0), fp = asInt(FP, 0), cp = asInt(CP, 0), pp = asInt(PP, 0), kp = asInt(KP, 0);
                if (sp != 0)
                    active.skillPoints += sp;
                if (fp != 0)
                    active.strengthPoints += fp;
                if (cp != 0)
                    active.commercePoints += cp;
                if (pp != 0)
                    active.progressPoints += pp;
                if (kp != 0)
                    active.victoryPoints += kp;
                active.placeCard(row, col, this);
                return true;
            }
        }

        // 4) Pure action cards (Basic intro handful):
        // We keep these very small; most are handled in Server (events/Brigand etc.)
        if ("Action".equalsIgnoreCase(placement) || "Action".equalsIgnoreCase(type)) {
            // e.g., Merchant Caravan: “gain 2 of your choice by discarding any 2 resources”
            if (nmEquals(nm, "Merchant Caravan")) {
                if (active.totalAllResources() < 2) {
                    active.sendMessage("You need at least 2 resources to play Merchant Caravan.");
                    return false;
                }
                // Just let player pick 2 to discard, then 2 to gain
                // (opting out by discarding and gaining same resource is allowed)
                for (int i = 0; i < 2; i++) {
                    active.sendMessage(
                            "PROMPT: Type Discard resource #" + (i + 1) + " [Brick|Grain|Lumber|Wool|Ore|Gold]:");
                    String g = active.receiveMessage();
                    active.removeResource(g, 1);
                }
                for (int i = 0; i < 2; i++) {
                    active.sendMessage(
                            "PROMPT: Type Gain resource #" + (i + 1) + " [Brick|Grain|Lumber|Wool|Ore|Gold]:");
                    String g = active.receiveMessage();
                    active.gainResource(g);
                }
                return true;
            }

            if (nmEquals(nm, "Scout")) {
                // Only meaningful when used with a new settlement (Server stores
                // lastSettlementRow/Col)
                active.flags.add("SCOUT_NEXT_SETTLEMENT");
                return true;
            }

            if (nmEquals(nm, "Brigitta the Wise Woman")) {
                // Choose production die result before rolling; we store forced value in Server
                active.flags.add("BRIGITTA");
                return true;
            }

            // Discard 3 gold and take any 2 resources of your choice in return.
            if (nmEquals(nm, "Goldsmith")) {
                if (!active.removeResource("Gold", 3)) {
                    active.sendMessage("Goldsmith: you need 3 Gold to play this.");
                    return false;
                }
                active.sendMessage("Goldsmith: choose two resources to gain:");
                for (int i = 1; i <= 2; i++) {
                    active.sendMessage("Pick resource #" + i + " [Brick|Grain|Lumber|Wool|Ore|Gold]:");
                    String g = active.receiveMessage();
                    active.gainResource(g);
                }
                return true;
            }

            // Swap 2 of your own Regions OR 2 of your own Expansion cards.
            // Stored resources on Regions remain on the same cards; placement rules must
            // hold.
            if (nmEquals(nm, "Relocation")) {
                active.sendMessage(
                        "PROMPT: Relocation - Type 'REGION' to swap two regions or 'EXP' to swap two expansions:");
                String pick = active.receiveMessage();
                boolean swapRegions = (pick != null && pick.trim().toUpperCase().startsWith("R"));
                boolean swapExp = (pick != null && pick.trim().toUpperCase().startsWith("E"));
                if (!swapRegions && !swapExp) {
                    active.sendMessage("Relocation canceled (need REGION or EXP).");
                    return false;
                }

                // Read two coordinates
                active.sendMessage("PROMPT: Enter first coordinate (row col):");
                int r1 = 0, c1 = 0;
                try {
                    String[] t = active.receiveMessage().trim().split("\\s+");
                    r1 = Integer.parseInt(t[0]);
                    c1 = Integer.parseInt(t[1]);
                } catch (Exception e) {
                    active.sendMessage("Invalid coordinate.");
                    return false;
                }
                active.sendMessage("PROMPT: Enter second coordinate (row col):");
                int r2 = 0, c2 = 0;
                try {
                    String[] t = active.receiveMessage().trim().split("\\s+");
                    r2 = Integer.parseInt(t[0]);
                    c2 = Integer.parseInt(t[1]);
                } catch (Exception e) {
                    active.sendMessage("Invalid coordinate.");
                    return false;
                }

                Card a = active.getCard(r1, c1);
                Card b = active.getCard(r2, c2);
                if (a == null || b == null) {
                    active.sendMessage("Relocation: both positions must contain cards.");
                    return false;
                }

                if (swapRegions) {
                    if (!isRegionCard(a) || !isRegionCard(b)) {
                        active.sendMessage("Relocation (Region): both cards must be Regions.");
                        return false;
                    }
                    // target slots must be valid region slots (i.e., not center)
                    if (isCenterSlot(r1) || isCenterSlot(r2)) {
                        active.sendMessage("Relocation: regions must be outside center row.");
                        return false;
                    }
                    // Swap without re-applying effects
                    active.placeCard(r1, c1, b);
                    active.placeCard(r2, c2, a);
                    active.sendMessage("Relocation done (Regions swapped).");
                    return true;
                } else { // swapExp
                    if (!isExpansionCard(a) || !isExpansionCard(b)) {
                        active.sendMessage("Relocation (Expansion): both cards must be expansions.");
                        return false;
                    }
                    // Must still obey expansion placement for each target slot
                    if (!isAboveOrBelowSettlementOrCity(active, r2, c2)
                            || !isAboveOrBelowSettlementOrCity(active, r1, c1)) {
                        active.sendMessage("Relocation: target slot is not valid for an expansion.");
                        return false;
                    }
                    active.placeCard(r1, c1, b);
                    active.placeCard(r2, c2, a);
                    active.sendMessage("Relocation done (Expansions swapped).");
                    return true;
                }
            }

            // Default: treat as “+1 VP”
            active.victoryPoints += 1;
            active.sendMessage("Played " + nm + ": +1 VP (default).");
            return true;
        }

        // Fallback: accept placement (ugly default)
        active.placeCard(row, col, this);
        return true;
    }

    // ----- tiny helpers used above -----
    private boolean isRegionCard(Card c) {
        return c != null && c.type != null && c.type.equalsIgnoreCase("Region");
    }

    private boolean isExpansionCard(Card c) {
        if (c == null)
            return false;
        String pl = (c.placement == null ? "" : c.placement.toLowerCase());
        return pl.contains("expansion");
    }
}