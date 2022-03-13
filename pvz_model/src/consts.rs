use crate::TodParticleDefinition;

#[repr(i32)]
pub enum AdviceType {
    ClickOnSun = 0,
    ClickedOnSun = 1,
    ClickedOnCoin = 2,
    SeedRefresh = 3,
    CantAffordPlant = 4,
    PlantGravebustersOnGraves = 5,
    PlantLilypadOnWater = 6,
    PlantTanglekelpOnWater = 7,
    PlantSeashroomOnWater = 8,
    PlantPotatoeMineOnLily = 9,
    PlantWrongArtType = 10,
    PlantNeedPot = 11,
    PlantNotOnGrave = 12,
    PlantNotOnCrater = 13,
    CantPlantThere = 14,
    PlantNotOnWater = 15,
    PlantingNeedsGround = 16,
    BeghouledDragToMatch3 = 17,
    BeghouledMatch3 = 18,
    BeghouledMatch4 = 19,
    BeghouledSaveSun = 20,
    BeghouledUseCrater1 = 21,
    BeghouledUseCrater2 = 22,
    PlantNotPassedLine = 23,
    PlantOnlyOnRepeaters = 24,
    PlantOnlyOnMelonpult = 25,
    PlantOnlyOnSunflower = 26,
    PlantOnlyOnSpikeweed = 27,
    PlantOnlyOnKernelpult = 28,
    PlantOnlyOnMagnetshroom = 29,
    PlantOnlyOnFumeshroom = 30,
    PlantOnlyOnLilypad = 31,
    PlantNeedsRepeater = 32,
    PlantNeedsMelonpult = 33,
    PlantNeedsSunflower = 34,
    PlantNeedsSpikeweed = 35,
    PlantNeedsKernelpult = 36,
    PlantNeedsMagnetshroom = 37,
    PlantNeedsFumeshroom = 38,
    PlantNeedsLilypad = 39,
    SlotMachinePull = 40,
    HugeWave = 41,
    ShovelRefresh = 42,
    PortalRelocating = 43,
    SlotMachineCollectSun = 44,
    DestoryPotsToFinisihLevel = 45,
    UseShovelOnPots = 46,
    AlmostThere = 47,
    ZombiquariumClickTrophy = 48,
    ZombiquariumCollectSun = 49,
    ZombiquariumClickToFeed = 50,
    ZombiquariumBuySnorkel = 51,
    IZombiePlantsNotReal = 52,
    IZombieNotPassedLine = 53,
    IZombieLeftOfLine = 54,
    SlotMachineSpinAgain = 55,
    IZombieEatAllBrains = 56,
    PeashooterDied = 57,
    StinkySleeping = 58,
    BeghouledNoMoves = 59,
    PlantSunflower5 = 60,
    PlantingNeedSleeping = 61,
    ClickToContinue = 62,
    SurviveFlags = 63,
    UnlockedMode = 64,
    NumAdviceTypes = 65,
    None = -1,
}

#[repr(i32)]
pub enum AnimType {
    None = 0,
    Once = 1,
    Pingpong = 2,
    Loop = 3,
}

#[repr(i32)]
pub enum AwardType {
    ForLevel = 0,
    CreditsZombieNote = 1,
}

#[repr(i32)]
pub enum BackgroundType {
    Day = 0,
    Night = 1,
    Pool = 2,
    Fog = 3,
    Roof = 4,
    Boss = 5,
    MushroomGarden = 6,
    GreenHouse = 7,
    Zombiquarium = 8,
    TreeOfWisdom = 9,
}

#[repr(i32)]
pub enum BoardResult {
    None = 0,
    Won = 1,
    Lost = 2,
    Restart = 3,
    Quit = 4,
    QuitApp = 5,
    Cheat = 6,
}

#[repr(i32)]
pub enum ChallengePage {
    Survival = 0,
    Challenge = 1,
    Limbo = 2,
    Puzzle = 3,
    MaxChallangePages = 4,
}

#[repr(i32)]
pub enum ChallengeState {
    Normal = 0,
    BeghouledMoving = 1,
    BeghouledFalling = 2,
    BeghouledNoMatches = 3,
    SlotMachineRolling = 4,
    StormFlash1 = 5,
    StormFlash2 = 6,
    StormFlash3 = 7,
    ZenFading = 8,
    ScaryPotterMalleting = 9,
    LastStandOnslaught = 10,
    TreeJustGrew = 11,
    TreeGiveWisdom = 12,
    TreeWaitingToBabble = 13,
    TreeBabbling = 14,
}

#[repr(i32)]
pub enum ChosenSeedState {
    FlyingToBank = 0,
    InBank = 1,
    FlyingToChooser = 2,
    InChooser = 3,
    PacketHidden = 4,
}

#[repr(i32)]
pub enum CoinMotion {
    FromSky = 0,
    FromSkySlow = 1,
    FromPlant = 2,
    Coin = 3,
    LawnmowerCoin = 4,
    FromPresent = 5,
    FromBoss = 6,
}

#[repr(i32)]
pub enum CoinType {
    None = 0,
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    Smallsun = 5,
    Bigsun = 6,
    FinalSeedPacket = 7,
    Trophy = 8,
    Shovel = 9,
    Almanac = 10,
    Carkeys = 11,
    Vase = 12,
    WateringCan = 13,
    Taco = 14,
    Note = 15,
    UsableSeedPacket = 16,
    PresentPlant = 17,
    AwardMoneyBag = 18,
    AwardPresent = 19,
    AwardBagDiamond = 20,
    AwardSilverSunflower = 21,
    AwardGoldSunflower = 22,
    Chocolate = 23,
    AwardChocolate = 24,
    PresentMinigames = 25,
    PresentPuzzleMode = 26,
}

#[repr(i32)]
pub enum CrazyDaveState {
    Off = 0,
    Entering = 1,
    Leaving = 2,
    Idling = 3,
    Talking = 4,
    HandingTalking = 5,
    HandingIdling = 6,
}

#[repr(i32)]
pub enum CreditsPhase {
    Main1 = 0,
    Main2 = 1,
    Main3 = 2,
    End = 3,
}

#[repr(i32)]
pub enum CursorType {
    Normal = 0,
    PlantFromBank = 1,
    PlantFromUsableCoin = 2,
    PlantFromGlove = 3,
    PlantFromDuplicator = 4,
    PlantFromWheelBarrow = 5,
    Shovel = 6,
    Hammer = 7,
    CobcannonTarget = 8,
    WateringCan = 9,
    Fertilizer = 10,
    BugSpray = 11,
    Phonograph = 12,
    Chocolate = 13,
    Glove = 14,
    MoneySign = 15,
    Wheelbarrow = 16,
    TreeFood = 17,
}

#[repr(i32)]
pub enum DebugTextMode {
    None = 0,
    ZombieSpawn = 1,
    Music = 2,
    Memory = 3,
    Collision = 4,
}

#[repr(i32)]
pub enum DrawVariation {
    Normal = 0,
    Imitater = 1,
    MarigoldWhite = 2,
    MarigoldMagenta = 3,
    MarigoldOrange = 4,
    MarigoldPink = 5,
    MarigoldLightBlue = 6,
    MarigoldRed = 7,
    MarigoldViolet = 9,
    MarigoldLavender = 10,
    MarigoldYellow = 11,
    MarigoldLightGreen = 12,
    ZenGarden = 13,
    ZenGardenWater = 14,
    SproutNoFlowre = 15,
    ImitaterLess = 16,
    Aquarium = 17,
}

#[repr(i32)]
pub enum EffectType {
    Particle = 0,
    Trail = 1,
    Reanim = 2,
    Attachment = 3,
    Other = 4,
}

#[repr(i32)]
pub enum EmitterType {
    Circle = 0,
    Box = 1,
    BoxPath = 2,
    CirclePath = 3,
    CircleEvenSpacing = 4,
}

#[repr(i32)]
pub enum EResult {
    Done = 0,
    NotStarted = 1,
    NotCompleted = 2,
    NotFound = 3,
    HttpError = 4,
    HttpRedirect = 5,
    Aborted = 6,
    SocketError = 7,
    InvalidAddr = 8,
    ConnectFail = 9,
    Disconnected = 10,
    InternalError = 11,
}

#[repr(i32)]
pub enum FilterEffect {
    WashedOut = 0,
    LessWashedOut = 1,
    White = 2,
    NumFilterEffects = 3,
    None = -1,
}

#[repr(i32)]
pub enum GameMode {
    Adventure = 0,
    SurvivalNormalStage1 = 1,
    SurvivalNormalStage2 = 2,
    SurvivalNormalStage3 = 3,
    SurvivalNormalStage4 = 4,
    SurvivalNormalStage5 = 5,
    SurvivalHardStage1 = 6,
    SurvivalHardStage2 = 7,
    SurvivalHardStage3 = 8,
    SurvivalHardStage4 = 9,
    SurvivalHardStage5 = 10,
    SurvivalEndlessStage1 = 11,
    SurvivalEndlessStage2 = 12,
    SurvivalEndlessStage3 = 13,
    SurvivalEndlessStage4 = 14,
    SurvivalEndlessStage5 = 15,
    ChallengeWarAndPeas = 16,
    ChallengeWallnutBowling = 17,
    ChallengeSlotMachine = 18,
    ChallengeRainingSeeds = 19,
    ChallengeBeghouled = 20,
    ChallengeInvisighoul = 21,
    ChallengeSeeingStars = 22,
    ChallengeZombiquarium = 23,
    ChallengeBeghouledTwist = 24,
    ChallengeLittleTrouble = 25,
    ChallengePortalCombat = 26,
    ChallengeColumn = 27,
    ChallengeBobsledBonanza = 28,
    ChallengeSpeed = 29,
    ChallengeWhackAZombie = 30,
    ChallengeLastStand = 31,
    ChallengeWarAndPeas2 = 32,
    ChallengeWallnutBowling2 = 33,
    ChallengePogoParty = 34,
    ChallengeFinalBoss = 35,
    ChallengeArtChallenge1 = 36,
    ChallengeSunnyDay = 37,
    ChallengeResodded = 38,
    ChallengeBigTime = 39,
    ChallengeArtChallenge2 = 40,
    ChallengeAirRaid = 41,
    ChallengeIce = 42,
    ChallengeZenGarden = 43,
    ChallengeHighGravity = 44,
    ChallengeGraveDanger = 45,
    ChallengeShovel = 46,
    ChallengeStormyNight = 47,
    ChallengeBungeeBlitz = 48,
    ChallengeSquirrel = 49,
    TreeOfWisdom = 50,
    ScaryPotter1 = 51,
    ScaryPotter2 = 52,
    ScaryPotter3 = 53,
    ScaryPotter4 = 54,
    ScaryPotter5 = 55,
    ScaryPotter6 = 56,
    ScaryPotter7 = 57,
    ScaryPotter8 = 58,
    ScaryPotter9 = 59,
    ScaryPotterEndless = 60,
    PuzzleIZombie1 = 61,
    PuzzleIZombie2 = 62,
    PuzzleIZombie3 = 63,
    PuzzleIZombie4 = 64,
    PuzzleIZombie5 = 65,
    PuzzleIZombie6 = 66,
    PuzzleIZombie7 = 67,
    PuzzleIZombie8 = 68,
    PuzzleIZombie9 = 69,
    PuzzleIZombieEndless = 70,
    NumGameModes = 71,
}

#[repr(i32)]
pub enum GameScenes {
    Loading = 0,
    Menu = 1,
    LevelIntro = 2,
    Playing = 3,
    ZombiesWon = 4,
    Award = 5,
    Credit = 6,
    Challenge = 7,
}

#[repr(i32)]
pub enum GardenType {
    Main = 0,
    Mushroom = 1,
    Wheelbarrow = 2,
    Aquarium = 3,
}

#[repr(i32)]
pub enum GridItemType {
    None = 0,
    Gravestone = 1,
    Crater = 2,
    Ladder = 3,
    PortalCircle = 4,
    PortalSquare = 5,
    Brain = 6,
    ScaryPot = 7,
    Squirrel = 8,
    ZenTool = 9,
    Stinky = 10,
    Rake = 11,
    IzombieBrain = 12,
}

#[repr(i32)]
pub enum GridItemState {
    Normal = 0,
    GravestoneSpecial = 1,
    PortalClosed = 2,
    ScaryPotQuestion = 3,
    ScaryPotLeaf = 4,
    ScaryPotZombie = 5,
    SquirrelWaiting = 6,
    SquirrelPeeking = 7,
    SquirrelRunningUp = 8,
    SquirrelRunningDown = 9,
    SquirrelRunningLeft = 10,
    SquirrelRunningRight = 11,
    SquirrelCaught = 12,
    SquirrelZombie = 13,
    ZenToolWateringCan = 14,
    ZenToolFertilizer = 15,
    ZenToolBugSpray = 16,
    ZenToolPhonograph = 17,
    ZenToolGoldWateringCan = 18,
    StinkyWalkingLeft = 19,
    StinkyTurningLeft = 20,
    StinkyWalkingRight = 21,
    StinkyTurningRight = 22,
    StinkySleeping = 23,
    StinkyFallingAsleep = 24,
    StinkyWakingUp = 25,
    RakeAttracting = 26,
    RakeWaiting = 27,
    RakeTriggered = 28,
    BrainSquished = 29,
}

#[repr(i32)]
pub enum GridSquareType {
    None = 0,
    Grass = 1,
    Dirt = 2,
    Pool = 3,
    HighGround = 4,
}

#[repr(i32)]
pub enum HelmType {
    None = 0,
    TrafficCone = 1,
    Pail = 2,
    Football = 3,
    Digger = 4,
    Redeyes = 5,
    Headband = 6,
    Bobsled = 7,
    Wallnut = 8,
    Tallnut = 9,
}

#[repr(i32)]
pub enum KeyCode {
    Unknown = 0,
    Lbutton = 1,
    Rbutton = 2,
    Cancel = 3,
    Mbutton = 4,
    Back = 8,
    Tab = 9,
    Clear = 12,
    Return = 13,
    Shift = 16,
    Control = 17,
    Menu = 18,
    Pause = 19,
    Capital = 20,
    Hangul = 21,
    Kana = 22,
    Junja = 23,
    Final = 24,
    Kanji = 25,
    Hanja = 26,
    Escape = 27,
    Convert = 28,
    Nonconvert = 29,
    Accept = 30,
    Modechange = 31,
    Space = 32,
    Prior = 33,
    Next = 34,
    End = 35,
    Home = 36,
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
    Select = 41,
    Print = 42,
    Execute = 43,
    Snapshot = 44,
    Insert = 45,
    Delete = 46,
    Help = 47,
    Asciibegin = 48,
    Asciiend = 90,
    Lwin = 91,
    Rwin = 92,
    Apps = 93,
    Numpad0 = 96,
    Numpad1 = 97,
    Numpad2 = 98,
    Numpad3 = 99,
    Numpad4 = 100,
    Numpad5 = 101,
    Numpad6 = 102,
    Numpad7 = 103,
    Numpad8 = 104,
    Numpad9 = 105,
    Multiply = 106,
    Add = 107,
    Separator = 108,
    Subtract = 109,
    Decimal = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    Numlock = 144,
    Scroll = 145,
    Asciibegin2 = 179,
    Asciiend2 = 224,
}

#[repr(i32)]
pub enum LawnMowerState {
    RollingIn = 0,
    Ready = 1,
    Triggered = 2,
    Squished = 3,
}

#[repr(i32)]
pub enum LawnMowerType {
    Lawn = 0,
    Pool = 1,
    Roof = 2,
    SuperMower = 3,
}

#[repr(i32)]
pub enum MagnetItemType {
    None = 0,
    Pail1 = 1,
    Pail2 = 2,
    Pail3 = 3,
    FootballHelmet1 = 4,
    FootballHelmet2 = 5,
    FootballHelmet3 = 6,
    Door1 = 7,
    Door2 = 8,
    Door3 = 9,
    Pogo1 = 10,
    Pogo2 = 11,
    Pogo3 = 12,
    JackInTheBox = 13,
    Ladder1 = 14,
    Ladder2 = 15,
    Ladder3 = 16,
    LadderPlaced = 17,
    SilverCoin = 18,
    GoldCoin = 19,
    Diamond = 20,
    PickAxe = 21,
}

#[repr(i32)]
pub enum MessageStyle {
    Off = 0,
    TutorialLevel1 = 1,
    TutorialLevel1Stay = 2,
    TutorialLevel2 = 3,
    TutorialLater = 4,
    TutorialLaterStay = 5,
    HintLong = 6,
    HintFast = 7,
    HintStay = 8,
    HintTallFast = 9,
    HintTall10Seconds = 10,
    HintTall8Seconds = 11,
    HintTallLong = 12,
    BigMiddle = 13,
    BigMiddleFast = 14,
    HouseName = 15,
    HugeWave = 16,
    SlotMachine = 17,
    ZenGardenLong = 18,
}

#[repr(i32)]
pub enum MowerHeight {
    Land = 0,
    DownToPool = 1,
    InPool = 2,
    UpToLand = 3,
}

#[repr(i32)]
pub enum MusicBurstState {
    Off = 0,
    Starting = 1,
    On = 2,
    Finishing = 3,
}

#[repr(i32)]
pub enum MusicDrumsState {
    Off = 0,
    OnQueued = 1,
    On = 2,
    OffQueued = 3,
    Fading = 4,
}

#[repr(i32)]
pub enum MusicFile {
    MainMusic = 1,
    Drums = 2,
    Hihats = 3,
    CreditsZombiesOnYourLawn = 4,
    NumMusicFiles = 5,
    None = -1,
}

#[repr(i32)]
pub enum MusicTune {
    DayGrasswalk = 1,
    NightMoongrains = 2,
    PoolWaterygraves = 3,
    FogRigormormist = 4,
    RoofGrazetheroof = 5,
    ChooseYourSeeds = 6,
    TitleCrazyDaveMainTheme = 7,
    ZenGarden = 8,
    PuzzleCerebrawl = 9,
    MinigameLoonboon = 10,
    Conveyer = 11,
    FinalBossBrainiacManiac = 12,
    CreditsZombiesOnYourLawn = 13,
    NumMusicTunes = 14,
    None = -1,
}

#[repr(i32)]
pub enum ParticleEffect {
    Melonsplash = 0,
    Wintermelon = 1,
    Fumecloud = 2,
    Popcornsplash = 3,
    Powie = 4,
    Jackexplode = 5,
    ZombieHead = 6,
    ZombieArm = 7,
    ZombieTrafficCone = 8,
    ZombiePail = 9,
    ZombieHelmet = 10,
    ZombieFlag = 11,
    ZombieDoor = 12,
    ZombieNewspaper = 13,
    ZombieHeadlight = 14,
    Pow = 15,
    ZombiePogo = 16,
    ZombieNewspaperHead = 17,
    ZombieBalloonHead = 18,
    SodRoll = 19,
    GraveStoneRise = 20,
    Planting = 21,
    PlantingPool = 22,
    ZombieRise = 23,
    GraveBuster = 24,
    GraveBusterDie = 25,
    PoolSplash = 26,
    IceSparkle = 27,
    SeedPacket = 28,
    TallNutBlock = 29,
    Doom = 30,
    DiggerRise = 31,
    DiggerTunnel = 32,
    DancerRise = 33,
    PoolSparkly = 34,
    WallnutEatSmall = 35,
    WallnutEatLarge = 36,
    PeaSplat = 37,
    SpikeSplat = 38,
    ButterSplat = 39,
    CabbageSplat = 40,
    PuffSplat = 41,
    StarSplat = 42,
    IceTrap = 43,
    SnowpeaSplat = 44,
    SnowpeaPuff = 45,
    SnowpeaTrail = 46,
    LanternShine = 47,
    SeedPacketPickup = 48,
    PotatoMine = 49,
    PotatoMineRise = 50,
    PuffshroomTrail = 51,
    PuffshroomMuzzle = 52,
    SeedPacketFlash = 53,
    WhackAZombieRise = 54,
    ZombieLadder = 55,
    UmbrellaReflect = 56,
    SeedPacketPick = 57,
    IceTrapZombie = 58,
    IceTrapRelease = 59,
    ZamboniSmoke = 60,
    Gloomcloud = 61,
    ZombiePogoHead = 62,
    ZamboniTire = 63,
    ZamboniExplosion = 64,
    ZamboniExplosion2 = 65,
    CatapultExplosion = 66,
    MowerCloud = 67,
    BossIceBall = 68,
    Blastmark = 69,
    CoinPickupArrow = 70,
    PresentPickup = 71,
    ImitaterMorph = 72,
    MoweredZombieHead = 73,
    MoweredZombieArm = 74,
    ZombieHeadPool = 75,
    ZombieBossFireball = 76,
    FireballDeath = 77,
    IceballDeath = 78,
    IceballTrail = 79,
    FireballTrail = 80,
    BossExplosion = 81,
    ScreenFlash = 82,
    TrophySparkle = 83,
    PortalCircle = 84,
    PortalSquare = 85,
    PottedPlantGlow = 86,
    PottedWaterPlantGlow = 87,
    PottedZenGlow = 88,
    MindControl = 89,
    VaseShatter = 90,
    VaseShatterLeaf = 91,
    VaseShatterZombie = 92,
    AwardPickupArrow = 93,
    ZombieSeaweed = 94,
    ZombieMustache = 95,
    ZombieFutureGlasses = 96,
    Pinata = 97,
    DustSquash = 98,
    DustFoot = 99,
    Daisy = 100,
    CreditStrobe = 101,
    CreditsRayswipe = 102,
    CreditsZombieheadwipe = 103,
    Starburst = 104,
    CreditsFog = 105,
    NumParticles = 106,
    None = -1,
}

#[repr(i32)]
pub enum ParticleFieldType {
    Invalid = 0,
    Friction = 1,
    Acceleration = 2,
    Attractor = 3,
    MaxVelocity = 4,
    Velocity = 5,
    Position = 6,
    SystemPosition = 7,
    GroundConstraint = 8,
    Shake = 9,
    Circle = 10,
    Away = 11,
    ParticleFieldCount = 12,
}

#[repr(i32)]
pub enum PlantRowType {
    Dirt = 0,
    Normal = 1,
    Pool = 2,
    HighGround = 3,
}

#[repr(i32)]
pub enum PlantState {
    Notready = 0,
    Ready = 1,
    Doingspecial = 2,
    SquashLook = 3,
    SquashPreLaunch = 4,
    SquashRising = 5,
    SquashFalling = 6,
    SquashDoneFalling = 7,
    GravebusterLanding = 8,
    GravebusterEating = 9,
    ChomperBiting = 10,
    ChomberBitingGotOne = 11,
    ChomperBitingMissed = 12,
    ChomperDigesting = 13,
    ChomperSwallowing = 14,
    PotatoRising = 15,
    PotatoArmed = 16,
    PotatoMashed = 17,
    SpikeweedAttacking = 18,
    SpikeweedAttacking2 = 19,
    ScaredyshroomLowering = 20,
    ScaredyshroomScared = 21,
    ScaredyshroomRaising = 22,
    SunshroomSmall = 23,
    SunshroomGrowing = 24,
    SunshroomBig = 25,
    MagnetshroomSucking = 26,
    MagnetshroomCharging = 27,
    BowlingUp = 28,
    BowlingDown = 29,
    CactusLow = 30,
    CactusRising = 31,
    CactusHigh = 32,
    CactusLowering = 33,
    TanglekelpGrabbing = 34,
    CobcannonArming = 35,
    CobcannonLoading = 36,
    CobcannonReady = 37,
    CobcannonFiring = 38,
    KernelpultButter = 39,
    UmbrellaTriggered = 40,
    UmbrellaReflecting = 41,
    ImitaterMorphing = 42,
    ZenGardenWatered = 43,
    ZenGardenNeedy = 44,
    ZenGardenHappy = 45,
    MarigoldEnding = 46,
    FlowerpotInvulnerable = 47,
    LilypadInvulnerable = 48,
}

#[repr(i32)]
pub enum PlantSubClass {
    Normal = 0,
    Shooter = 1,
}

#[repr(i32)]
pub enum PlantWeapon {
    Primary = 0,
    Secondary = 1,
}

#[repr(i32)]
pub enum PottedPlantAge {
    Sprout = 0,
    Small = 1,
    Medium = 2,
    Full = 3,
}

#[repr(i32)]
pub enum PottedPlantNeed {
    None = 0,
    Water = 1,
    Fertilizer = 2,
    Bugspray = 3,
    Phonograph = 4,
}

#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,
    Lobbed = 1,
    Threepeater = 2,
    Bee = 3,
    BeeBackwards = 4,
    Puff = 5,
    Backwards = 6,
    Star = 7,
    FloatOver = 8,
    Homing = 9,
}

#[repr(i32)]
pub enum ProjectileType {
    Pea = 0,
    Snowpea = 1,
    Cabbage = 2,
    Melon = 3,
    Puff = 4,
    Wintermelon = 5,
    Fireball = 6,
    Star = 7,
    Spike = 8,
    Basketball = 9,
    Kernel = 10,
    Cobbig = 11,
    Butter = 12,
    ZombiePea = 13,
}

#[repr(i32)]
pub enum ReanimationType {
    LoadbarSprout = 0,
    LoadbarZombiehead = 1,
    Sodroll = 2,
    FinalWave = 3,
    Peashooter = 4,
    Wallnut = 5,
    Lilypad = 6,
    Sunflower = 7,
    Lawnmower = 8,
    Readysetplant = 9,
    Cherrybomb = 10,
    Squash = 11,
    Doomshroom = 12,
    Snowpea = 13,
    Repeater = 14,
    Sunshroom = 15,
    Tallnut = 16,
    Fumeshroom = 17,
    Puffshroom = 18,
    Hypnoshroom = 19,
    Chomper = 20,
    Zombie = 21,
    Sun = 22,
    Potatomine = 23,
    Spikeweed = 24,
    Spikerock = 25,
    Threepeater = 26,
    Marigold = 27,
    Iceshroom = 28,
    ZombieFootball = 29,
    ZombieNewspaper = 30,
    ZombieZamboni = 31,
    Splash = 32,
    Jalapeno = 33,
    JalapenoFire = 34,
    CoinSilver = 35,
    ZombieCharred = 36,
    ZombieCharredImp = 37,
    ZombieCharredDigger = 38,
    ZombieCharredZamboni = 39,
    ZombieCharredCatapult = 40,
    ZombieCharredGargantuar = 41,
    Scrareyshroom = 42,
    Pumpkin = 43,
    Plantern = 44,
    Torchwood = 45,
    Splitpea = 46,
    Seashroom = 47,
    Blover = 48,
    FlowerPot = 49,
    Cactus = 50,
    Dancer = 51,
    Tanglekelp = 52,
    Starfruit = 53,
    Polevaulter = 54,
    Balloon = 55,
    Gargantuar = 56,
    Imp = 57,
    Digger = 58,
    DiggerDirt = 59,
    ZombieDolphinrider = 60,
    Pogo = 61,
    BackupDancer = 62,
    Bobsled = 63,
    Jackinthebox = 64,
    Snorkel = 65,
    Bungee = 66,
    Catapult = 67,
    Ladder = 68,
    Puff = 69,
    Sleeping = 70,
    GraveBuster = 71,
    ZombiesWon = 72,
    Magnetshroom = 73,
    Boss = 74,
    Cabbagepult = 75,
    Kernelpult = 76,
    Melonpult = 77,
    Coffeebean = 78,
    Umbrellaleaf = 79,
    Gatlingpea = 80,
    Cattail = 81,
    Gloomshroom = 82,
    BossIceball = 83,
    BossFireball = 84,
    Cobcannon = 85,
    Garlic = 86,
    GoldMagnet = 87,
    WinterMelon = 88,
    TwinSunflower = 89,
    PoolCleaner = 90,
    RoofCleaner = 91,
    FirePea = 92,
    Imitater = 93,
    Yeti = 94,
    BossDriver = 95,
    LawnMoweredZombie = 96,
    CrazyDave = 97,
    TextFadeOn = 98,
    Hammer = 99,
    SlotMachineHandle = 100,
    CreditsFootball = 101,
    CreditsJackbox = 102,
    SelectorScreen = 103,
    PortalCircle = 104,
    PortalSquare = 105,
    ZengardenSprout = 106,
    ZengardenWateringcan = 107,
    ZengardenFertilizer = 108,
    ZengardenBugspray = 109,
    ZengardenPhonograph = 110,
    Diamond = 111,
    ZombieHand = 112,
    Stinky = 113,
    Rake = 114,
    RainCircle = 115,
    RainSplash = 116,
    ZombieSurprise = 117,
    CoinGold = 118,
    TreeOfWisdom = 119,
    TreeOfWisdomClouds = 120,
    TreeFood = 121,
    CreditsMain = 122,
    CreditsMain2 = 123,
    CreditsMain3 = 124,
    ZombieCreditsDance = 125,
    CreditsStage = 126,
    CreditsBigbrain = 127,
    CreditsFlowerPetals = 128,
    CreditsInfantry = 129,
    CreditsThroat = 130,
    CreditsCrazydave = 131,
    CreditsBossdance = 132,
    ZombieCreditsScreenDoor = 133,
    ZombieCreditsConehead = 134,
    CreditsZombiearmy1 = 135,
    CreditsZombiearmy2 = 136,
    CreditsTombstones = 137,
    CreditsSolarpower = 138,
    CreditsAnyhour = 139,
    CreditsWearetheundead = 140,
    CreditsDiscolights = 141,
    NumReanims = 142,
    None = -1,
}

#[repr(i32)]
pub enum ReanimLoopType {
    Loop = 0,
    LoopFullLastFrame = 1,
    PlayOnce = 2,
    PlayOnceAndHold = 3,
    PlayOnceFullLastFrame = 4,
    PlayOnceFullLastFrameAndHold = 5,
}

#[repr(i32)]
pub enum ScaryPotType {
    None = 0,
    Seed = 1,
    Zombie = 2,
    Sun = 3,
}

#[repr(i32)]
pub enum SeedType {
    Peashooter = 0,
    Sunflower = 1,
    Cherrybomb = 2,
    Wallnut = 3,
    Potatomine = 4,
    Snowpea = 5,
    Chomper = 6,
    Repeater = 7,
    Puffshroom = 8,
    Sunshroom = 9,
    Fumeshroom = 10,
    Gravebuster = 11,
    Hypnoshroom = 12,
    Scaredyshroom = 13,
    Iceshroom = 14,
    Doomshroom = 15,
    Lilypad = 16,
    Squash = 17,
    Threepeater = 18,
    Tanglekelp = 19,
    Jalapeno = 20,
    Spikeweed = 21,
    Torchwood = 22,
    Tallnut = 23,
    Seashroom = 24,
    Plantern = 25,
    Cactus = 26,
    Blover = 27,
    Splitpea = 28,
    Starfruit = 29,
    Pumpkinshell = 30,
    Magnetshroom = 31,
    Cabbagepult = 32,
    Flowerpot = 33,
    Kernelpult = 34,
    InstantCoffee = 35,
    Garlic = 36,
    Umbrella = 37,
    Marigold = 38,
    Melonpult = 39,
    Gatlingpea = 40,
    Twinsunflower = 41,
    Gloomshroom = 42,
    Cattail = 43,
    Wintermelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    Cobcannon = 47,
    Imitater = 48,
    ExplodeONut = 49,
    GiantWallnut = 50,
    Sprout = 51,
    Leftpeater = 52,
    NumSeedTypes = 53,
    BeghouledButtonShuffle = 54,
    BeghouledButtonCrater = 55,
    SlotMachineSun = 56,
    SlotMachineDiamond = 57,
    ZombiquariumSnorkle = 58,
    ZombiquariumTrophy = 59,
    ZombieNormal = 60,
    ZombieTrafficCone = 61,
    ZombiePolevaulter = 62,
    ZombiePail = 63,
    ZombieLadder = 64,
    ZombieDigger = 65,
    ZombieBungee = 66,
    ZombieFootball = 67,
    ZombieBalloon = 68,
    ZombieScreenDoor = 69,
    Zomboni = 70,
    ZombiePogo = 71,
    ZombieDancer = 72,
    ZombieGargantuar = 73,
    ZombieImp = 74,
    None = -1,
}

#[repr(i32)]
pub enum SeedChooserState {
    Normal = 0,
    ViewLawn = 1,
}

#[repr(i32)]
pub enum SelectorAnimState {
    Open = 0,
    NewUser = 1,
    ShowSign = 2,
    Idle = 3,
}

#[repr(i32)]
pub enum ShieldType {
    None = 0,
    Door = 1,
    Newspaper = 2,
    Ladder = 3,
}

#[repr(i32)]
pub enum TodCurves {
    Constant = 0,
    Linear = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInOut = 4,
    EaseInOutWeak = 5,
    FastInOut = 6,
    FastInOutWeak = 7,
    WeakFastInOut = 8,
    Bounce = 9,
    BounceFastMiddle = 10,
    BounceSlowMiddle = 11,
    SinWave = 12,
    EaseSinWave = 13,
}

#[repr(i32)]
pub enum TrailType {
    Ice = 0,
    None = -1,
}

#[repr(i32)]
pub enum TutorialState {
    Off = 0,
    Level1PickUpPeashooter = 1,
    Level1PlantPeashooter = 2,
    Level1RefreshPeashooter = 3,
    Level1Completed = 4,
    Level2PickUpSunflower = 5,
    Level2PlantSunflower = 6,
    Level2RefreshSunflower = 7,
    Level2Completed = 8,
    MoresunPickUpSunflower = 9,
    MoresunPlantSunflower = 10,
    MoresunRefreshSunflower = 11,
    MoresunCompleted = 12,
    SlotMachinePull = 13,
    SlotMachineCompleted = 14,
    ShovelPickup = 15,
    ShovelDig = 16,
    ShovelKeepDigging = 17,
    ShovelCompleted = 18,
    ZombiquariumBuySnorkel = 19,
    ZombiquariumBoughtSnorkel = 20,
    ZombiquariumClickTrophy = 21,
    ZenGardenPickupWater = 22,
    ZenGardenWaterPlant = 23,
    ZenGardenKeepWatering = 24,
    ZenGardenVisitStore = 25,
    ZenGardenFertilizePlants = 26,
    ZenGardenCompleted = 27,
    WhackAZombieBeforePickSeed = 28,
    WhackAZombiePickSeed = 29,
    WhackAZombieCompleted = 30,
}

#[repr(i32)]
pub enum UnlockingState {
    Off = 0,
    Shaking = 1,
    Fading = 2,
}

#[repr(i32)]
pub enum ZombieAttackType {
    Chew = 0,
    DriveOver = 1,
    Vault = 2,
    Ladder = 3,
}

#[repr(i32)]
pub enum ZombieHeight {
    ZombieNormal = 0,
    InToPool = 1,
    OutOfPool = 2,
    DraggedUnder = 3,
    UpToHighGround = 4,
    DownOffHighGround = 5,
    UpLadder = 6,
    Falling = 7,
    InToChimney = 8,
    GettingBungeeDropped = 9,
    Zombiquarium = 10,
}

#[repr(i32)]
#[derive(PartialEq)]
pub enum ZombiePhase {
    ZombieNormal = 0,
    ZombieDying = 1,
    ZombieBurned = 2,
    ZombieMowered = 3,
    BungeeDiving = 4,
    BungeeDivingScreaming = 5,
    BungeeAtBottom = 6,
    BungeeGrabbing = 7,
    BungeeRising = 8,
    BungeeHitOuchy = 9,
    BungeeCutscene = 10,
    PolevaulterPreVault = 11,
    PolevaulterInVault = 12,
    PolevaulterPostVault = 13,
    RisingFromGrave = 14,
    JackInTheBoxRunning = 15,
    JackInTheBoxPopping = 16,
    BobsledSliding = 17,
    BobsledBoarding = 18,
    BobsledCrashing = 19,
    PogoBouncing = 20,
    PogoHighBounce1 = 21,
    PogoHighBounce2 = 22,
    PogoHighBounce3 = 23,
    PogoHighBounce4 = 24,
    PogoHighBounce5 = 25,
    PogoHighBounce6 = 26,
    PogoForwardBounce2 = 27,
    PogoForwardBounce7 = 28,
    NewspaperReading = 29,
    NewspaperMaddening = 30,
    NewspaperMad = 31,
    DiggerTunneling = 32,
    DiggerRising = 33,
    DiggerTunnelingPauseWithoutAxe = 34,
    DiggerRiseWithoutAxe = 35,
    DiggerStunned = 36,
    DiggerWalking = 37,
    DiggerWalkingWithoutAxe = 38,
    DiggerCutscene = 39,
    DancerDancingIn = 40,
    DancerSnappingFingers = 41,
    DancerSnappingFingersWithLight = 42,
    DancerSnappingFingersHold = 43,
    DancerDancingLeft = 44,
    DancerWalkToRaise = 45,
    DancerRaiseLeft1 = 46,
    DancerRaiseRight1 = 47,
    DancerRaiseLeft2 = 48,
    DancerRaiseRight2 = 49,
    DancerRising = 50,
    DolphinWalking = 51,
    DolphinIntoPool = 52,
    DolphinRiding = 53,
    DolphinInJump = 54,
    DolphinWalkingInPool = 55,
    DolphinWalkingWithoutDolphin = 56,
    SnorkelWalking = 57,
    SnorkelIntoPool = 58,
    SnorkelWalkingInPool = 59,
    SnorkelUpToEat = 60,
    SnorkelEatingInPool = 61,
    SnorkelDownFromEat = 62,
    ZombiquariumAccel = 63,
    ZombiquariumDrift = 64,
    ZombiquariumBackAndForth = 65,
    ZombiquariumBite = 66,
    CatapultLaunching = 67,
    CatapultReloading = 68,
    GargantuarThrowing = 69,
    GargantuarSmashing = 70,
    ImpGettingThrown = 71,
    ImpLanding = 72,
    BalloonFlying = 73,
    BalloonPopping = 74,
    BalloonWalking = 75,
    LadderCarrying = 76,
    LadderPlacing = 77,
    BossEnter = 78,
    BossIdle = 79,
    BossSpawning = 80,
    BossStomping = 81,
    BossBungeesEnter = 82,
    BossBungeesDrop = 83,
    BossBungeesLeave = 84,
    BossDropRv = 85,
    BossHeadEnter = 86,
    BossHeadIdleBeforeSpit = 87,
    BossHeadIdleAfterSpit = 88,
    BossHeadSpit = 89,
    BossHeadLeave = 90,
    YetiRunning = 91,
    SquashPreLaunch = 92,
    SquashRising = 93,
    SquashFalling = 94,
    SquashDoneFalling = 95,
}

#[repr(i32)]
pub enum ZombieType {
    Normal = 0,
    Flag = 1,
    TrafficCone = 2,
    Polevaulter = 3,
    Pail = 4,
    Newspaper = 5,
    Door = 6,
    Football = 7,
    Dancer = 8,
    BackupDancer = 9,
    DuckyTube = 10,
    Snorkel = 11,
    Zamboni = 12,
    Bobsled = 13,
    DolphinRider = 14,
    JackInTheBox = 15,
    Balloon = 16,
    Digger = 17,
    Pogo = 18,
    Yeti = 19,
    Bungee = 20,
    Ladder = 21,
    Catapult = 22,
    Gargantuar = 23,
    Imp = 24,
    Boss = 25,
    PeaHead = 26,
    WallnutHead = 27,
    JalapenoHead = 28,
    GatlingHead = 29,
    SquashHead = 30,
    TallnutHead = 31,
    RedeyeGargantuar = 32,
    Zombatar = 33,
    Invalid = -1,
}

pub(crate) const G_PARTICLE_DEF_ARRAY: *const *const TodParticleDefinition =
    0x6A9F0C as *const *const TodParticleDefinition;
