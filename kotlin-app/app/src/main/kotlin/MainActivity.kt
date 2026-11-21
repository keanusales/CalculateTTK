package com.delta.ttk

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.KeyboardArrowUp
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.focus.FocusRequester
import androidx.compose.ui.focus.focusRequester
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import java.util.Locale
import kotlin.math.ceil

data class TtkRow(val partName: String, val ttks: List<String>)

data class TtkResult(val punishment: String, val drops: List<String>, val rows: List<TtkRow>)

val bodyParts = listOf("Head", "Chest", "Abdomen", "Arms", "Forearms", "Thighs", "Legs")

val damageParser = Regex("""(\d+(?:\.\d+)?)\*(\d+(?:\.\d+)?)""")

fun parseDamage(damageValue: String): Float {
  val stripped = damageValue.replace("\\s".toRegex(), "").replace(",", ".")
  val matchResult = damageParser.matchEntire(stripped)

  return if (matchResult != null) {
    val (first, second) = matchResult.destructured
    first.toFloat() * second.toFloat()
  } else stripped.toFloat()
}

fun parseDrops(dropsValue: String): List<Float> {
  return dropsValue.replace(",", ".").split("\\s+".toRegex())
    .filter { it.isNotBlank() }.map { it.toFloat() }.sortedDescending()
}

fun calculateTtk(damages: List<Float>, drops: List<Float>, rate: Float): TtkResult {
  require(drops.isNotEmpty() && drops.all { it > 0f && it <= 1f }) {
    "Ensure all drops values are between 0 and 1 (0, 1]."
  }
  require(damages.isNotEmpty() && damages.all { it > 0f }) {
    "Ensure all damages values are settled and positive."
  }
  require(rate > 0) { "Ensure the firerate value are positive." }

  val punish = 60000f / rate

  fun getTtk(damage: Float, drop: Float): String {
    val rawTtk = ((ceil((100f / damage / drop).toDouble()) - 1) * punish)
    return String.format(Locale.US, "%.2f", rawTtk)
  }

  val resultRows = bodyParts.zip(damages).map { (part, damage) ->
    val ttksForPart = drops.map { drop -> getTtk(damage, drop) }
    TtkRow(partName = part, ttks = ttksForPart)
  }

  val fpunish = String.format(Locale.US, "%.2f", punish)
  val fdrops = drops.map { "${it}" }

  return TtkResult(punishment = fpunish, drops = fdrops, rows = resultRows)
}

@Composable
fun TtkCalculatorScreen() {
  val damageInputs = remember { mutableStateListOf(*Array(bodyParts.size) { TextFieldValue("") }) }

  var dropInput by remember { mutableStateOf("") }
  var rateInput by remember { mutableStateOf("") }

  var result by remember { mutableStateOf<TtkResult?>(null) }
  var errorMessage by remember { mutableStateOf<String?>(null) }

  val focusRequesters = remember { List(bodyParts.size) { FocusRequester() } }

  Column(
    modifier = Modifier.fillMaxSize().safeDrawingPadding().verticalScroll(rememberScrollState()),
    verticalArrangement = Arrangement.spacedBy(12.dp)
  ) {
    Text(
      text = "Delta Force TTK Calculator", fontSize = 24.sp, fontWeight = FontWeight.Bold,
      modifier = Modifier.padding(top = 16.dp, bottom = 8.dp).padding(horizontal = 16.dp)
    )

    bodyParts.forEachIndexed { index, partName ->
      OutlinedTextField(
        value = damageInputs[index],
        onValueChange = { damageInputs[index] = it },
        label = { Text("Damage value for $partName") },
        modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp).focusRequester(focusRequesters[index]),
        singleLine = true,
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Phone, imeAction = ImeAction.Next),
        trailingIcon = {
          Row {
            IconButton(
              onClick = {
                val textToCopy = damageInputs[index].text
                damageInputs[index - 1] = TextFieldValue(
                  text = textToCopy, selection = TextRange(textToCopy.length)
                )
                focusRequesters[index - 1].requestFocus()
              },
              enabled = index > 0
            ) { Icon(Icons.Default.KeyboardArrowUp, contentDescription = "Up") }
            IconButton(
              onClick = {
                val textToCopy = damageInputs[index].text
                damageInputs[index + 1] = TextFieldValue(
                  text = textToCopy, selection = TextRange(textToCopy.length)
                )
                focusRequesters[index + 1].requestFocus()
              },
              enabled = index < bodyParts.lastIndex
            ) { Icon(Icons.Default.KeyboardArrowDown, contentDescription = "Down") }
          }
        }
      )
    }

    OutlinedTextField(
      value = dropInput,
      onValueChange = { dropInput = it },
      label = { Text("Damage drops (space separated)") },
      modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
      singleLine = true,
      keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Next)
    )

    OutlinedTextField(
      value = rateInput,
      onValueChange = { rateInput = it },
      label = { Text("Weapon firerate (RPM)") },
      modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
      singleLine = true,
      keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number, imeAction = ImeAction.Done)
    )

    Button(
      onClick = {
        try {
          errorMessage = null
          val damages = damageInputs.map { parseDamage(it.text) }
          val drops = parseDrops(dropInput)
          val rate = rateInput.replace(",", ".").toFloat()
          result = calculateTtk(damages, drops, rate)
        } catch (e: Exception) {
          errorMessage = "Erro nos dados: ${e.message}"
          result = null
        }
      },
      modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp)
    ) { Text("Calculate") }

    errorMessage?.let { msg ->
      Text(
        text = msg, color = Color.Red, fontWeight = FontWeight.Bold,
        modifier = Modifier.padding(horizontal = 16.dp)
      )
    }

    result?.let { ttkResult ->
      Card(
        modifier = Modifier.fillMaxWidth().padding(top = 16.dp, bottom = 24.dp),
        shape = androidx.compose.ui.graphics.RectangleShape,
        elevation = CardDefaults.cardElevation(defaultElevation = 0.dp)
      ) {
        Column(modifier = Modifier.padding(16.dp)) {
          Text(
            text = "Punishment is ${ttkResult.punishment} ms",
            fontWeight = FontWeight.Bold,
            modifier = Modifier.align(Alignment.CenterHorizontally).padding(bottom = 12.dp)
          )

          Row(modifier = Modifier.horizontalScroll(rememberScrollState())) {
            Column {
              Text("Part", fontWeight = FontWeight.Bold, modifier = Modifier.padding(8.dp))
              ttkResult.rows.forEach { row ->
                Text(row.partName, modifier = Modifier.padding(8.dp))
              }
            }

            ttkResult.drops.forEachIndexed { index, dropVal ->
              Column {
                Text(
                  text = "${dropVal}x",
                  fontWeight = FontWeight.Bold,
                  modifier = Modifier.padding(8.dp)
                )
                ttkResult.rows.forEach { row ->
                  Text(text = row.ttks[index], modifier = Modifier.padding(8.dp))
                }
              }
            }
          }
        }
      }
    }
  }
}

class MainActivity : ComponentActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    setContent {
      val useDarkTheme = isSystemInDarkTheme()
      val colors = if (useDarkTheme) darkColorScheme() else lightColorScheme()
      MaterialTheme(colorScheme = colors) {
        Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
          TtkCalculatorScreen()
        }
      }
    }
  }
}