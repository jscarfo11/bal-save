# import re


# exp = re.compile(r'"(\S*)"')

# with open('src/defaults.rs', 'r') as file:
#     content = file.read()
#     matches = exp.findall(content)
#     matches = [f'("{match}", false, false, false),' for match in matches]
#     matches = '\n'.join(matches)
#     print(matches
#     )

x = """v_blank: true
v_overstock_norm: true
v_reroll_surplus: true
v_palette: true
v_planet_tycoon: true
v_nacho_tong: true
v_telescope: true
v_seed_money: true
v_retcon: true
v_grabber: true
v_omen_globe: true
v_petroglyph: true
v_overstock_plus: true
v_liquidation: true
v_hieroglyph: true
v_crystal_ball: true
v_wasteful: true
v_planet_merchant: true
v_reroll_glut: true
v_paint_brush: true
v_tarot_merchant: true
v_antimatter: true
v_magic_trick: true
v_recyclomancy: true
v_tarot_tycoon: true
v_observatory: true
v_directors_cut: true
v_illusion: true
v_glow_up: true
v_hone: true
v_clearance_sale: true
v_money_tree: true"""

x = x.split('\n')
x = [i.split(':')[0] for i in x]
x = ['("' + i + '", false, false, false),' for i in x]
x = '\n'.join(x)
with open('default.txt', 'w') as file:
    file.write(x)
print(x)